use futures::future::join_all;
use itertools::Itertools;
use std::{
    collections::HashMap,
    io,
    path::Path,
    process::{Command, ExitStatus},
    sync::Arc,
};
use tempdir::TempDir;
use thiserror::Error;

use crate::{
    build::{Build, BuildBehaviour, BuildError},
    config::{Config, LuaVersion, LuaVersionUnset},
    lockfile::{LocalPackage, LocalPackageId, PinnedState},
    lua_installation::LuaInstallation,
    lua_rockspec::{RemoteLuaRockspec, RockspecFormat},
    operations::{get_all_dependencies, SearchAndDownloadError},
    package::PackageReq,
    path::Paths,
    progress::{MultiProgress, Progress, ProgressBar},
    remote_package_db::{RemotePackageDB, RemotePackageDBError},
    rockspec::RemoteRockspec,
    tree::Tree,
};

#[derive(Error, Debug)]
pub enum LuaRocksError {
    #[error(transparent)]
    LuaVersionUnset(#[from] LuaVersionUnset),
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[derive(Error, Debug)]
pub enum LuaRocksInstallError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    BuildError(#[from] BuildError),
}

#[derive(Error, Debug)]
pub enum InstallBuildDependenciesError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    RemotePackageDBError(#[from] RemotePackageDBError),
    #[error(transparent)]
    SearchAndDownloadError(#[from] SearchAndDownloadError),
    #[error(transparent)]
    BuildError(#[from] BuildError),
}

#[derive(Error, Debug)]
pub enum ExecLuaRocksError {
    #[error(transparent)]
    LuaVersionUnset(#[from] LuaVersionUnset),
    #[error("could not write luarocks config: {0}")]
    WriteLuarocksConfigError(io::Error),
    #[error("failed to run luarocks: {0}")]
    Io(#[from] io::Error),
    #[error("executing luarocks compatibility layer failed.\nstatus: {status}\nstdout: {stdout}\nstderr: {stderr}")]
    CommandFailure {
        status: ExitStatus,
        stdout: String,
        stderr: String,
    },
}

pub struct LuaRocksInstallation {
    pub tree: Tree,
    pub config: Config,
}

pub(crate) const LUAROCKS_VERSION: &str = "3.11.1-1";

const LUAROCKS_ROCKSPEC: &str = "
rockspec_format = '3.0'
package = 'luarocks'
version = '3.11.1-1'
source = {
   url = 'git+https://github.com/luarocks/luarocks',
   tag = 'v3.11.1'
}
";

impl LuaRocksInstallation {
    pub fn new(config: &Config) -> Result<Self, LuaRocksError> {
        let config = config.clone().with_tree(config.luarocks_tree().clone());
        let luarocks_installation = Self {
            tree: Tree::new(config.luarocks_tree().clone(), LuaVersion::from(&config)?)?,
            config,
        };
        Ok(luarocks_installation)
    }

    pub fn tree(&self) -> &Tree {
        &self.tree
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub async fn ensure_installed(
        &self,
        progress: &Progress<ProgressBar>,
    ) -> Result<(), LuaRocksInstallError> {
        let mut lockfile = self.tree.lockfile()?.write_guard();

        let luarocks_req =
            PackageReq::new("luarocks".into(), Some(LUAROCKS_VERSION.into())).unwrap();

        if !self.tree.match_rocks(&luarocks_req)?.is_found() {
            let rockspec = RemoteLuaRockspec::new(LUAROCKS_ROCKSPEC).unwrap();
            let pkg = Build::new(&rockspec, &self.tree, &self.config, progress)
                .constraint(luarocks_req.version_req().clone().into())
                .build_remote()
                .await?;
            lockfile.add(&pkg);
        }

        Ok(())
    }

    pub async fn install_build_dependencies<R: RemoteRockspec>(
        &self,
        build_backend: &str,
        rocks: &R,
        progress_arc: Arc<Progress<MultiProgress>>,
    ) -> Result<(), InstallBuildDependenciesError> {
        let progress = Arc::clone(&progress_arc);
        let mut lockfile = self.tree.lockfile()?.write_guard();
        let bar = progress.map(|p| p.new_bar());
        let package_db = RemotePackageDB::from_config(&self.config, &bar).await?;
        bar.map(|b| b.finish_and_clear());
        let build_dependencies = match rocks.format() {
            Some(RockspecFormat::_1_0 | RockspecFormat::_2_0) => {
                // XXX: rockspec formats < 3.0 don't support `build_dependencies`,
                // so we have to fetch the build backend from the dependencies.
                rocks
                    .dependencies()
                    .current_platform()
                    .iter()
                    .filter(|dep| dep.name().to_string().contains(build_backend))
                    .cloned()
                    .collect_vec()
            }
            _ => rocks.build_dependencies().current_platform().to_vec(),
        }
        .into_iter()
        .map(|dep| (BuildBehaviour::NoForce, dep))
        .collect_vec();

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let pin = PinnedState::Unpinned;
        get_all_dependencies(
            tx,
            build_dependencies,
            pin,
            Arc::new(package_db),
            Arc::new(lockfile.clone()),
            &self.config,
            progress_arc,
        )
        .await?;

        let mut all_packages = HashMap::with_capacity(rx.len());
        while let Some(dep) = rx.recv().await {
            all_packages.insert(dep.spec.id(), dep);
        }

        let installed_packages = join_all(all_packages.clone().into_values().map(|install_spec| {
            let bar = progress.map(|p| {
                p.add(ProgressBar::from(format!(
                    "💻 Installing build dependency: {}",
                    install_spec.downloaded_rock.rockspec().package(),
                )))
            });
            let config = self.config.clone();
            let tree = self.tree.clone();
            tokio::spawn(async move {
                let rockspec = install_spec.downloaded_rock.rockspec();
                let pkg = Build::new(rockspec, &tree, &config, &bar)
                    .constraint(install_spec.spec.constraint())
                    .behaviour(install_spec.build_behaviour)
                    .build_remote()
                    .await?;

                bar.map(|b| b.finish_and_clear());

                Ok::<_, InstallBuildDependenciesError>((pkg.id(), pkg))
            })
        }))
        .await
        .into_iter()
        .flatten()
        .try_collect::<_, HashMap<LocalPackageId, LocalPackage>, _>()?;

        installed_packages.iter().for_each(|(id, pkg)| {
            lockfile.add(pkg);

            all_packages
                .get(id)
                .map(|pkg| pkg.spec.dependencies())
                .unwrap_or_default()
                .into_iter()
                .for_each(|dependency_id| {
                    lockfile.add_dependency(
                        pkg,
                        installed_packages
                            .get(dependency_id)
                            .expect("required dependency not found"),
                    );
                });
        });

        Ok(())
    }

    pub fn make(
        self,
        rockspec_path: &Path,
        build_dir: &Path,
        dest_dir: &Path,
        lua: &LuaInstallation,
    ) -> Result<(), ExecLuaRocksError> {
        let dest_dir_str = dest_dir.to_string_lossy().to_string();
        let rockspec_path_str = rockspec_path.to_string_lossy().to_string();
        let args = vec![
            "make",
            "--deps-mode",
            "none",
            "--tree",
            &dest_dir_str,
            &rockspec_path_str,
        ];
        self.exec(args, build_dir, lua)
    }

    fn exec(
        self,
        args: Vec<&str>,
        cwd: &Path,
        lua: &LuaInstallation,
    ) -> Result<(), ExecLuaRocksError> {
        let luarocks_paths = Paths::new(self.tree)?;
        // Ensure a pure environment so we can do parallel builds
        let temp_dir = TempDir::new("lux-run-luarocks").unwrap();
        let luarocks_config_content = format!(
            "
variables = {{
    LUA_LIBDIR = \"{0}\",
    LUA_INCDIR = \"{1}\",
    LUA_VERSION = \"{2}\",
    MAKE = \"{3}\",
}}
",
            lua.lib_dir.display(),
            lua.include_dir.display(),
            LuaVersion::from(&self.config)?,
            self.config.make_cmd(),
        );
        let luarocks_config = temp_dir.path().join("luarocks-config.lua");
        std::fs::write(luarocks_config.clone(), luarocks_config_content)
            .map_err(ExecLuaRocksError::WriteLuarocksConfigError)?;
        let output = Command::new("luarocks")
            .current_dir(cwd)
            .args(args)
            .env("PATH", luarocks_paths.path_prepended().joined())
            .env("LUA_PATH", luarocks_paths.package_path().joined())
            .env("LUA_CPATH", luarocks_paths.package_cpath().joined())
            .env("HOME", temp_dir.into_path())
            .env("LUAROCKS_CONFIG", luarocks_config)
            .output()?;
        if output.status.success() {
            Ok(())
        } else {
            Err(ExecLuaRocksError::CommandFailure {
                status: output.status,
                stdout: String::from_utf8_lossy(&output.stdout).into(),
                stderr: String::from_utf8_lossy(&output.stderr).into(),
            })
        }
    }
}
