use clap::Args;
use eyre::Result;
use lux_lib::{
    config::Config,
    lockfile::LocalPackage,
    operations::{self},
    project::Project,
};
use tracing::{span, Level};

#[derive(Args, Default)]
pub struct Build {
    /// Ignore the project's lockfile and don't create one.
    #[arg(long)]
    no_lock: bool,

    /// Build only the dependencies
    #[arg(long)]
    only_deps: bool,
}

/// Returns `Some` if the `only_deps` arg is set to `false`.
pub async fn build(data: Build, config: Config) -> Result<Option<LocalPackage>> {
    let span = span!(Level::TRACE, "building...");

    let guard = span.enter();
    let project = Project::current_or_err()?;
    let result = operations::BuildProject::new(&project, &config)
        .no_lock(data.no_lock)
        .only_deps(data.only_deps)
        .build()
        .await?;
    drop(guard);
    Ok(result)
}
