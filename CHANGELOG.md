# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## `lux-cli` - [0.13.1](https://github.com/teto/lux/compare/v0.13.0...v0.13.1) - 2025-08-05

### Other
- update Cargo.lock dependencies

## `lux-cli` - [0.13.0](https://github.com/nvim-neorocks/lux/compare/v0.12.1...v0.13.0) - 2025-08-03

### Added
- *(build)* [**breaking**] pass in `LuaInstallation`
- [**breaking**] manage lua installations internally

### Other
- *(build)* [**breaking**] tidy up builder

## `lux-lib` - [0.19.0](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.18.1...lux-lib-v0.19.0) - 2025-08-03

### Added
- *(lockfile)* diff-friendly entrypoints ([#948](https://github.com/nvim-neorocks/lux/pull/948))
- *(build)* [**breaking**] pass in `LuaInstallation`
- [**breaking**] manage lua installations internally

### Other
- *(build)* [**breaking**] tidy up builder

## `lux-cli` - [0.12.1](https://github.com/nvim-neorocks/lux/compare/v0.12.0...v0.12.1) - 2025-07-31

### Other
- release ([#943](https://github.com/nvim-neorocks/lux/pull/943))

## `lux-cli` - [0.12.1](https://github.com/nvim-neorocks/lux/compare/v0.12.0...v0.12.1) - 2025-07-31

### Other
- updated the following local packages: lux-lib

## `lux-cli` - [0.12.0](https://github.com/nvim-neorocks/lux/compare/v0.11.2...v0.12.0) - 2025-07-31

### Added
- *(cli)* [**breaking**] rename `check` -> `lint` ([#836](https://github.com/nvim-neorocks/lux/pull/836))

## `lux-lib` - [0.18.0](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.17.0...lux-lib-v0.18.0) - 2025-07-31

### Added
- *(cli)* [**breaking**] rename `check` -> `lint` ([#836](https://github.com/nvim-neorocks/lux/pull/836))

## `lux-cli` - [0.11.2](https://github.com/nvim-neorocks/lux/compare/v0.11.1...v0.11.2) - 2025-07-30

### Other
- update Cargo.toml dependencies

## `lux-lib` - [0.17.0](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.16.1...lux-lib-v0.17.0) - 2025-07-30

### Fixed
- *(rockspec)* [**breaking**] support list elements in install spec ([#939](https://github.com/nvim-neorocks/lux/pull/939))

## `lux-cli` - [0.11.1](https://github.com/nvim-neorocks/lux/compare/v0.11.0...v0.11.1) - 2025-07-25

### Other
- *(readme)* update feature list

## `lux-lib` - [0.16.1](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.16.0...lux-lib-v0.16.1) - 2025-07-25

### Added
- expand variables in `source` `.dir`, `.file` and `.tag` ([#915](https://github.com/nvim-neorocks/lux/pull/915))

### Fixed
- *(build)* support multiline scripts in `command` backend ([#918](https://github.com/nvim-neorocks/lux/pull/918))
- search upwards when substituting `$(REF)` ([#917](https://github.com/nvim-neorocks/lux/pull/917))

### Other
- *(readme)* update feature list

## `lux-cli` - [0.11.0](https://github.com/nvim-neorocks/lux/compare/v0.10.2...v0.11.0) - 2025-07-23

### Added
- [**breaking**] auto-generate `.luarc.json` ([#910](https://github.com/nvim-neorocks/lux/pull/910))

### Other
- move shared dependencies to workspace manifest ([#908](https://github.com/nvim-neorocks/lux/pull/908))

## `lux-lib` - [0.16.0](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.15.1...lux-lib-v0.16.0) - 2025-07-23

### Added
- [**breaking**] auto-generate `.luarc.json` ([#910](https://github.com/nvim-neorocks/lux/pull/910))

### Other
- move shared dependencies to workspace manifest ([#908](https://github.com/nvim-neorocks/lux/pull/908))

## `lux-cli` - [0.10.2](https://github.com/nvim-neorocks/lux/compare/v0.10.1...v0.10.2) - 2025-07-23

### Other
- release ([#901](https://github.com/nvim-neorocks/lux/pull/901))

## `lux-lib` - [0.15.1](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.15.0...lux-lib-v0.15.1) - 2025-07-23

### Added
- pretty-print generated lua code ([#907](https://github.com/nvim-neorocks/lux/pull/907))
- more detailed error message when variable substitution fails ([#905](https://github.com/nvim-neorocks/lux/pull/905))

## `lux-cli` - [0.10.2](https://github.com/nvim-neorocks/lux/compare/v0.10.1...v0.10.2) - 2025-07-22

### Other
- updated the following local packages: lux-lib

## `lux-cli` - [0.10.1](https://github.com/nvim-neorocks/lux/compare/v0.10.0...v0.10.1) - 2025-07-22

### Other
- update flake.lock ([#882](https://github.com/nvim-neorocks/lux/pull/882))

## `lux-lib` - [0.14.1](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.14.0...lux-lib-v0.14.1) - 2025-07-22

### Fixed
- incorrect install path when installing packed rock ([#896](https://github.com/nvim-neorocks/lux/pull/896))
- fall back to unzipped manifest on HEAD request ([#895](https://github.com/nvim-neorocks/lux/pull/895))

### Other
- *(deps)* bump nonempty from 0.11.0 to 0.12.0 ([#894](https://github.com/nvim-neorocks/lux/pull/894))
- update flake.lock ([#882](https://github.com/nvim-neorocks/lux/pull/882))
- clarify cross-compilation comment

## `lux-cli` - [0.10.0](https://github.com/nvim-neorocks/lux/compare/v0.9.1...v0.10.0) - 2025-07-21

### Added
- *(build)* [**breaking**] more output in verbose mode ([#876](https://github.com/nvim-neorocks/lux/pull/876))

### Fixed
- [**breaking**] support transitive build dependencies ([#883](https://github.com/nvim-neorocks/lux/pull/883))
- *(cli)* typo in help docs ([#872](https://github.com/nvim-neorocks/lux/pull/872))

### Other
- *(test-resources)* sample-projects subdirectory

## `lux-lib` - [0.14.0](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.13.2...lux-lib-v0.14.0) - 2025-07-21

### Added
- fall back to unzipped manifest ([#890](https://github.com/nvim-neorocks/lux/pull/890))
- *(build)* [**breaking**] more output in verbose mode ([#876](https://github.com/nvim-neorocks/lux/pull/876))

### Fixed
- *(pack)* write `rock_manifest` using luarocks structure ([#887](https://github.com/nvim-neorocks/lux/pull/887))
- *(install)* don't install build dependencies of binary rocks ([#888](https://github.com/nvim-neorocks/lux/pull/888))
- [**breaking**] support transitive build dependencies ([#883](https://github.com/nvim-neorocks/lux/pull/883))

### Other
- *(test-resources)* sample-projects subdirectory
- *(deps)* bump mlua from 0.10.3 to 0.10.5 ([#875](https://github.com/nvim-neorocks/lux/pull/875))

## `lux-cli` - [0.9.1](https://github.com/nvim-neorocks/lux/compare/v0.9.0...v0.9.1) - 2025-07-15

### Other
- release ([#867](https://github.com/nvim-neorocks/lux/pull/867))

## `lux-lib` - [0.13.2](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.13.1...lux-lib-v0.13.2) - 2025-07-15

### Fixed
- *(lux.toml)* bad conversion of deploy spec to lua ([#871](https://github.com/nvim-neorocks/lux/pull/871))

## `lux-cli` - [0.9.1](https://github.com/nvim-neorocks/lux/compare/v0.9.0...v0.9.1) - 2025-07-14

### Other
- updated the following local packages: lux-lib

## `lux-cli` - [0.9.0](https://github.com/nvim-neorocks/lux/compare/v0.8.3...v0.9.0) - 2025-07-14

### Fixed
- *(build)* [**breaking**] always install and use build dependencies ([#865](https://github.com/nvim-neorocks/lux/pull/865))
- *(uninstall)* prune dangling dependencies ([#864](https://github.com/nvim-neorocks/lux/pull/864))
- *(uninstall)* don't uninstall if operation is cancelled
- *(cli)* correct --lua-dir documentation

## `lux-lib` - [0.13.0](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.12.1...lux-lib-v0.13.0) - 2025-07-14

### Fixed
- *(build)* [**breaking**] always install and use build dependencies ([#865](https://github.com/nvim-neorocks/lux/pull/865))

## `lux-cli` - [0.8.3](https://github.com/nvim-neorocks/lux/compare/v0.8.2...v0.8.3) - 2025-07-12

### Other
- *(deps)* bump toml from 0.8.22 to 0.9.0 ([#846](https://github.com/nvim-neorocks/lux/pull/846))

## `lux-lib` - [0.12.1](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.12.0...lux-lib-v0.12.1) - 2025-07-12

### Fixed
- *(build)* relax `source.dir` inferring logic ([#859](https://github.com/nvim-neorocks/lux/pull/859))
- *(config)* TOML configs overridden by defaults ([#858](https://github.com/nvim-neorocks/lux/pull/858))

### Other
- *(deps)* bump zip from 4.2.0 to 4.3.0 ([#850](https://github.com/nvim-neorocks/lux/pull/850))
- *(deps)* bump toml_edit from 0.22.26 to 0.23.0 ([#847](https://github.com/nvim-neorocks/lux/pull/847))
- *(deps)* bump toml from 0.8.22 to 0.9.0 ([#846](https://github.com/nvim-neorocks/lux/pull/846))

## `lux-cli` - [0.8.2](https://github.com/nvim-neorocks/lux/compare/v0.8.1...v0.8.2) - 2025-07-08

### Added
- expose shell completions in main binary ([#837](https://github.com/nvim-neorocks/lux/pull/837))

### Other
- *(cli/completion)* auto-detect shell ([#845](https://github.com/nvim-neorocks/lux/pull/845))

## `lux-cli` - [0.8.1](https://github.com/nvim-neorocks/lux/compare/v0.8.0...v0.8.1) - 2025-07-08

### Added
- *(cli)* allow passing path to `fmt` ([#835](https://github.com/nvim-neorocks/lux/pull/835))

## `lux-lib` - [0.12.0](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.11.0...lux-lib-v0.12.0) - 2025-07-08

### Fixed
- *(build)* [**breaking**] `copy_directorys` drops subdirectories ([#842](https://github.com/nvim-neorocks/lux/pull/842))
- *(build)* install conf files to etc/conf ([#841](https://github.com/nvim-neorocks/lux/pull/841))

## `lux-cli` - [0.8.0](https://github.com/nvim-neorocks/lux/compare/v0.7.4...v0.8.0) - 2025-07-07

### Added
- *(cli)* lx shell ([#817](https://github.com/nvim-neorocks/lux/pull/817))
- add help for `lx lua` flags

### Fixed
- fix!(cli): `lx pack` broken in projects ([#821](https://github.com/nvim-neorocks/lux/pull/821))

### Other
- [**breaking**] `_prepended` for `PackagePath`
- `lx shell` cleanup
- *(deps)* bump tokio from 1.45.0 to 1.46.0 ([#827](https://github.com/nvim-neorocks/lux/pull/827))

## `lux-lib` - [0.11.0](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.10.1...lux-lib-v0.11.0) - 2025-07-07

### Added
- *(lux-lua)* state functions and search functionality ([#781](https://github.com/nvim-neorocks/lux/pull/781))
- use `--verbose` flag to enable compiler warnings ([#833](https://github.com/nvim-neorocks/lux/pull/833))
- *(install)* support rocks with only .src.rock sources ([#823](https://github.com/nvim-neorocks/lux/pull/823))

### Fixed
- fix!(cli): `lx pack` broken in projects ([#821](https://github.com/nvim-neorocks/lux/pull/821))
- *(build/command)* make `_command` fields optional ([#832](https://github.com/nvim-neorocks/lux/pull/832))

### Other
- *(build)* [**breaking**] don't expose `BuildBackend` trait ([#826](https://github.com/nvim-neorocks/lux/pull/826))
- *(build)* [**breaking**] use Builder pattern for `BuildBackend` trait ([#825](https://github.com/nvim-neorocks/lux/pull/825))
- [**breaking**] `_prepended` for `PackagePath`
- *(build)* [**breaking**] `lua_rockspec::Build` -> `build::backend::BuildBackend` ([#824](https://github.com/nvim-neorocks/lux/pull/824))
- *(deps)* bump tokio from 1.45.0 to 1.46.0 ([#827](https://github.com/nvim-neorocks/lux/pull/827))

## `lux-cli` - [0.7.4](https://github.com/nvim-neorocks/lux/compare/v0.7.3...v0.7.4) - 2025-06-27

### Added
- *(cli)* set `LUA_INIT` for `lx exec`
- feat!(cli): add `--no-loader` flag to repl and run commands

### Fixed
- only run repl initialisation in repl

### Other
- *(deps)* bump lua-src from 547.0.0 to 548.1.1 ([#782](https://github.com/nvim-neorocks/lux/pull/782))

## `lux-lib` - [0.10.1](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.10.0...lux-lib-v0.10.1) - 2025-06-27

### Added
- *(cli)* set `LUA_INIT` for `lx exec`
- feat!(cli): add `--no-loader` flag to repl and run commands

### Fixed
- only run repl initialisation in repl

### Other
- *(deps)* bump md5 from 0.7.0 to 0.8.0 ([#816](https://github.com/nvim-neorocks/lux/pull/816))
- *(deps)* bump zip from 4.1.0 to 4.2.0 ([#814](https://github.com/nvim-neorocks/lux/pull/814))
- *(deps)* bump lua-src from 547.0.0 to 548.1.1 ([#782](https://github.com/nvim-neorocks/lux/pull/782))

## `lux-cli` - [0.7.3](https://github.com/nvim-neorocks/lux/compare/v0.7.2...v0.7.3) - 2025-06-17

### Added
- *(repl)* add project to welcome message

### Fixed
- broken `lx lua --help`

## `lux-lib` - [0.10.0](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.9.2...lux-lib-v0.10.0) - 2025-06-17

### Added
- *(repl)* add project to welcome message

### Fixed
- [**breaking**] only alias `exit` to `os.exit()` in repl

### Other
- *(deps)* bump zip from 4.0.0 to 4.1.0 ([#800](https://github.com/nvim-neorocks/lux/pull/800))

## `lux-cli` - [0.7.2](https://github.com/nvim-neorocks/lux/compare/v0.7.1...v0.7.2) - 2025-06-16

### Other
- release ([#792](https://github.com/nvim-neorocks/lux/pull/792))

## `lux-cli` - [0.7.2](https://github.com/nvim-neorocks/lux/compare/v0.7.1...v0.7.2) - 2025-06-15

### Other
- updated the following local packages: lux-lib

## `lux-cli` - [0.7.1](https://github.com/nvim-neorocks/lux/compare/v0.7.0...v0.7.1) - 2025-06-14

### Added
- busted-nlua test backend ([#769](https://github.com/nvim-neorocks/lux/pull/769))

### Other
- *(licensing)* MIT -> LGPL-3.0+ ([#778](https://github.com/nvim-neorocks/lux/pull/778))

## `lux-lib` - [0.9.1](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.9.0...lux-lib-v0.9.1) - 2025-06-14

### Added
- busted-nlua test backend ([#769](https://github.com/nvim-neorocks/lux/pull/769))
- *(rockspec)* support `gitrec+` prefixes ([#786](https://github.com/nvim-neorocks/lux/pull/786))

### Other
- *(licensing)* MIT -> LGPL-3.0+ ([#778](https://github.com/nvim-neorocks/lux/pull/778))
- *(cargo.toml)* use repository instead of homepage ([#779](https://github.com/nvim-neorocks/lux/pull/779))

## `lux-cli` - [0.7.0](https://github.com/nvim-neorocks/lux/compare/v0.6.0...v0.7.0) - 2025-06-09

### Added
- [**breaking**] `--test` and `--build` flags for `lx lua` ([#774](https://github.com/nvim-neorocks/lux/pull/774))
- *(cli)* flag to override variables ([#765](https://github.com/nvim-neorocks/lux/pull/765))

### Fixed
- don't set `LUA_INIT` if lux-lua not present ([#763](https://github.com/nvim-neorocks/lux/pull/763))

### Other
- *(deps)* bump which from 7.0.3 to 8.0.0 ([#772](https://github.com/nvim-neorocks/lux/pull/772))
- refactor!(lua-rockspec): split out lua from dependencies ([#730](https://github.com/nvim-neorocks/lux/pull/730))

## `lux-lib` - [0.9.0](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.8.0...lux-lib-v0.9.0) - 2025-06-09

### Added
- [**breaking**] `--test` and `--build` flags for `lx lua` ([#774](https://github.com/nvim-neorocks/lux/pull/774))
- *(cli)* flag to override variables ([#765](https://github.com/nvim-neorocks/lux/pull/765))

### Fixed
- fix!(install): properly link transitive dependencies ([#771](https://github.com/nvim-neorocks/lux/pull/771))
- properly quote complex keys when generating rockspec
- don't set `LUA_INIT` if lux-lua not present ([#763](https://github.com/nvim-neorocks/lux/pull/763))
- *(build)* lua binaries not wrapped properly ([#766](https://github.com/nvim-neorocks/lux/pull/766))

### Other
- *(deps)* bump proptest from 1.6.0 to 1.7.0 ([#776](https://github.com/nvim-neorocks/lux/pull/776))
- *(deps)* bump which from 7.0.3 to 8.0.0 ([#772](https://github.com/nvim-neorocks/lux/pull/772))
- refactor!(lua-rockspec): split out lua from dependencies ([#730](https://github.com/nvim-neorocks/lux/pull/730))

## `lux-cli` - [0.6.0](https://github.com/nvim-neorocks/lux/compare/v0.5.3...v0.6.0) - 2025-06-01

### Added
- feat!(test): full test spec implementation ([#759](https://github.com/nvim-neorocks/lux/pull/759))
- [**breaking**] lux.toml source templates ([#704](https://github.com/nvim-neorocks/lux/pull/704))
- add .gitignore to install tree root ([#753](https://github.com/nvim-neorocks/lux/pull/753))
- keep lux-cli and lux-lua versions in sync ([#751](https://github.com/nvim-neorocks/lux/pull/751))
- feat!(cli/check): respect ignore files by default ([#749](https://github.com/nvim-neorocks/lux/pull/749))
- *(cli)* Allow passing args into `lx check` ([#746](https://github.com/nvim-neorocks/lux/pull/746))

### Fixed
- [**breaking**] more robust lua binary detection ([#757](https://github.com/nvim-neorocks/lux/pull/757))

## `lux-lib` - [0.8.0](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.7.0...lux-lib-v0.8.0) - 2025-06-01

### Added
- feat!(test): full test spec implementation ([#759](https://github.com/nvim-neorocks/lux/pull/759))
- [**breaking**] lux.toml source templates ([#704](https://github.com/nvim-neorocks/lux/pull/704))
- substitute variables from environment
- [**breaking**] make `HasVariables` trait `pub(crate)`
- add .gitignore to install tree root ([#753](https://github.com/nvim-neorocks/lux/pull/753))
- feat!(cli/check): respect ignore files by default ([#749](https://github.com/nvim-neorocks/lux/pull/749))

### Fixed
- [**breaking**] more robust lua binary detection ([#757](https://github.com/nvim-neorocks/lux/pull/757))
- [**breaking**] always wrap lua bin scripts ([#756](https://github.com/nvim-neorocks/lux/pull/756))

### Other
- follow-up fix for source url templates

## `lux-cli` - [0.5.3](https://github.com/nvim-neorocks/lux/compare/v0.5.2...v0.5.3) - 2025-05-25

### Fixed
- fix!(build/builtin): use external_dependency info
- properly capture command output

## `lux-lib` - [0.7.0](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.6.2...lux-lib-v0.7.0) - 2025-05-25

### Fixed
- fix!(build/builtin): use external_dependency info
- properly capture command output
- variable substitution for `LUA_BINDIR`
- external_dependencies variable substitutions
- rock_manifest parsing error
- external dependencies not finding libraries via pkg-config
- fall back to `all.rock` when downloading packed rocks
- add checks to prevent trying to unpack HTML response ([#735](https://github.com/nvim-neorocks/lux/pull/735))
- [**breaking**] bin scripts installed into tree's root ([#724](https://github.com/nvim-neorocks/lux/pull/724))

### Other
- add gnum4 to devShell
- *(deps)* bump zip from 3.0.0 to 4.0.0 ([#728](https://github.com/nvim-neorocks/lux/pull/728))

## `lux-cli` - [0.5.2](https://github.com/nvim-neorocks/lux/compare/v0.5.1...v0.5.2) - 2025-05-21

### Fixed
- unable to parse large luarocks manifest ([#726](https://github.com/nvim-neorocks/lux/pull/726))

### Other
- *(deps)* upgrade ([#712](https://github.com/nvim-neorocks/lux/pull/712))

## `lux-lib` - [0.6.2](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.6.1...lux-lib-v0.6.2) - 2025-05-21

### Fixed
- unable to parse large luarocks manifest ([#726](https://github.com/nvim-neorocks/lux/pull/726))

### Other
- *(deps)* upgrade ([#712](https://github.com/nvim-neorocks/lux/pull/712))

## `lux-cli` - [0.5.1](https://github.com/nvim-neorocks/lux/compare/v0.5.0...v0.5.1) - 2025-05-16

### Other
- update Cargo.lock dependencies

## `lux-lib` - [0.6.1](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.6.0...lux-lib-v0.6.1) - 2025-05-16

### Fixed
- error when luajit is not aliased to lua ([#707](https://github.com/nvim-neorocks/lux/pull/707))

### Other
- *(deps)* bump zip from 2.6.0 to 3.0.0 ([#705](https://github.com/nvim-neorocks/lux/pull/705))

## `lux-cli` - [0.5.0](https://github.com/nvim-neorocks/lux/compare/v0.4.5...v0.5.0) - 2025-05-14

### Added
- [**breaking**] separate project from config ([#692](https://github.com/nvim-neorocks/lux/pull/692))

### Fixed
- [**breaking**] luajit version autodetection + prevent manifest download if lua version not detected ([#702](https://github.com/nvim-neorocks/lux/pull/702))

### Other
- *(readme)* add packaging status badge ([#698](https://github.com/nvim-neorocks/lux/pull/698))
- [**breaking**] unify `Install` tree operations

## `lux-lib` - [0.6.0](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.5.0...lux-lib-v0.6.0) - 2025-05-14

### Added
- more detailed zip error messages ([#701](https://github.com/nvim-neorocks/lux/pull/701))
- [**breaking**] separate project from config ([#692](https://github.com/nvim-neorocks/lux/pull/692))

### Fixed
- [**breaking**] luajit version autodetection + prevent manifest download if lua version not detected ([#702](https://github.com/nvim-neorocks/lux/pull/702))
- remove hash field from `RockSourceInternal` ([#697](https://github.com/nvim-neorocks/lux/pull/697))

### Other
- *(readme)* add packaging status badge ([#698](https://github.com/nvim-neorocks/lux/pull/698))
- [**breaking**] unify `Install` tree operations

## `lux-cli` - [0.4.5](https://github.com/nvim-neorocks/lux/compare/v0.4.4...v0.4.5) - 2025-05-13

### Added
- *(cli)* autogenerate a .gitignore file ([#684](https://github.com/nvim-neorocks/lux/pull/684))

## `lux-lib` - [0.5.0](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.4.1...lux-lib-v0.5.0) - 2025-05-13

### Fixed
- [**breaking**] treat unknown string versions as `< SemVer` ([#689](https://github.com/nvim-neorocks/lux/pull/689))
# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## `lux-cli` - [0.4.1](https://github.com/nvim-neorocks/lux/compare/v0.4.0...v0.4.1) - 2025-05-11

### Other
- updated the following local packages: lux-lib

## `lux-cli` - [0.4.0](https://github.com/nvim-neorocks/lux/compare/v0.3.15...v0.4.0) - 2025-05-10

### Added
- use pkg-config to probe lux-lua
- *(cli)* `lx add` for git dependencies ([#667](https://github.com/nvim-neorocks/lux/pull/667))

### Other
- [**breaking**] unify `Sync` by making it take in a `Project`

## `lux-lib` - [0.4.0](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.3.12...lux-lib-v0.4.0) - 2025-05-10

### Added
- *(cli)* `lx update` for git dependencies ([#671](https://github.com/nvim-neorocks/lux/pull/671))
- use pkg-config to probe lux-lua
- *(cli)* `lx add` for git dependencies ([#667](https://github.com/nvim-neorocks/lux/pull/667))

### Fixed
- *(project)* use project tree instead of tree provided in configuration
- *(cli)* fields removed on update

### Other
- [**breaking**] unify `Sync` by making it take in a `Project`

## `lux-cli` - [0.3.15](https://github.com/nvim-neorocks/lux/compare/v0.3.14...v0.3.15) - 2025-05-09

### Added
- *(cli)* nicer error messages

### Fixed
- *(cli)* rough UX on luajit

### Other
- *(deps)* bump tokio from 1.44.0 to 1.45.0 ([#659](https://github.com/nvim-neorocks/lux/pull/659))
- add git dependencies to comparison table

## `lux-lib` - [0.3.12](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.3.11...lux-lib-v0.3.12) - 2025-05-09

### Added
- more Lua coverage + Lua tests

### Fixed
- *(cli)* rough UX on luajit

### Other
- *(deps)* bump tokio from 1.44.0 to 1.45.0 ([#659](https://github.com/nvim-neorocks/lux/pull/659))
- add git dependencies to comparison table
- *(deps)* bump luajit-src from 210.5.11+97813fb to 210.5.12+a4f56a4 ([#656](https://github.com/nvim-neorocks/lux/pull/656))

## `lux-cli` - [0.3.14](https://github.com/nvim-neorocks/lux/compare/v0.3.13...v0.3.14) - 2025-05-01

### Added
- git dependencies for local projects ([#644](https://github.com/nvim-neorocks/lux/pull/644))
- *(lib/install)* support installing from alternate sources ([#624](https://github.com/nvim-neorocks/lux/pull/624))

### Fixed
- *(build)* dependencies added as install tree entrypoints ([#651](https://github.com/nvim-neorocks/lux/pull/651))
- *(build)* transitive dependencies added as dependencies of main package

### Other
- refactor!(lux-lib): builder for `PackageInstallSpec` ([#629](https://github.com/nvim-neorocks/lux/pull/629))

## `lux-lib` - [0.3.11](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.3.10...lux-lib-v0.3.11) - 2025-05-01

### Added
- git dependencies for local projects ([#644](https://github.com/nvim-neorocks/lux/pull/644))
- *(lib/install)* support installing from alternate sources ([#624](https://github.com/nvim-neorocks/lux/pull/624))

### Fixed
- *(build)* dependencies added as install tree entrypoints ([#651](https://github.com/nvim-neorocks/lux/pull/651))
- *(build)* unpacking tar archive can panic ([#649](https://github.com/nvim-neorocks/lux/pull/649))

### Other
- refactor!(lux-lib): builder for `PackageInstallSpec` ([#629](https://github.com/nvim-neorocks/lux/pull/629))

## `lux-cli` - [0.3.13](https://github.com/nvim-neorocks/lux/compare/v0.3.12...v0.3.13) - 2025-04-29

### Other
- update Cargo.lock dependencies

## `lux-lib` - [0.3.10](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.3.9...lux-lib-v0.3.10) - 2025-04-29

### Added
- *(lux-lib)* more lenient dev version parsing ([#623](https://github.com/nvim-neorocks/lux/pull/623))

### Fixed
- parse versions without a contraint prefix as == ([#640](https://github.com/nvim-neorocks/lux/pull/640))

### Other
- *(deps)* bump insta from 1.42.0 to 1.43.0 ([#642](https://github.com/nvim-neorocks/lux/pull/642))

## `lux-cli` - [0.3.12](https://github.com/nvim-neorocks/lux/compare/v0.3.11...v0.3.12) - 2025-04-27

### Fixed
- *(cli)* suggest `--no-lock` instead of `--ignore-lockfile`

## `lux-cli` - [0.3.11](https://github.com/nvim-neorocks/lux/compare/v0.3.10...v0.3.11) - 2025-04-27

### Fixed
- conflicting external dependency spec parse error ([#632](https://github.com/nvim-neorocks/lux/pull/632))

## `lux-lib` - [0.3.9](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.3.8...lux-lib-v0.3.9) - 2025-04-27

### Fixed
- conflicting external dependency spec parse error ([#632](https://github.com/nvim-neorocks/lux/pull/632))

## `lux-cli` - [0.3.10](https://github.com/nvim-neorocks/lux/compare/v0.3.9...v0.3.10) - 2025-04-23

### Other
- *(deps)* bump stylua from 2.0.2 to 2.1.0 ([#621](https://github.com/nvim-neorocks/lux/pull/621))

## `lux-cli` - [0.3.9](https://github.com/nvim-neorocks/lux/compare/v0.3.8...v0.3.9) - 2025-04-22

### Other
- *(deps)* bump stylua from 2.0.0 to 2.0.2 ([#619](https://github.com/nvim-neorocks/lux/pull/619))

## `lux-cli` - [0.3.8](https://github.com/nvim-neorocks/lux/compare/v0.3.7...v0.3.8) - 2025-04-21

### Added
- windows msvc toolchain support ([#501](https://github.com/nvim-neorocks/lux/pull/501))
- `lx generate-rockspec`

### Fixed
- lockfile entries removed after `lx add` ([#617](https://github.com/nvim-neorocks/lux/pull/617))

## `lux-lib` - [0.3.8](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.3.7...lux-lib-v0.3.8) - 2025-04-21

### Added
- windows msvc toolchain support ([#501](https://github.com/nvim-neorocks/lux/pull/501))

### Fixed
- *(manifest)* re-download if corrupted

### Other
- update flake.lock ([#615](https://github.com/nvim-neorocks/lux/pull/615))

## `lux-cli` - [0.3.7](https://github.com/nvim-neorocks/lux/compare/v0.3.6...v0.3.7) - 2025-04-16

### Other
- updated the following local packages: lux-lib

## `lux-cli` - [0.3.6](https://github.com/nvim-neorocks/lux/compare/v0.3.5...v0.3.6) - 2025-04-14

### Other
- use compilation target to get platform identifier ([#597](https://github.com/nvim-neorocks/lux/pull/597))

## `lux-lib` - [0.3.6](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.3.5...lux-lib-v0.3.6) - 2025-04-14

### Fixed
- *(pack)* regression in manifest creation ([#599](https://github.com/nvim-neorocks/lux/pull/599))

### Other
- use compilation target to get platform identifier ([#597](https://github.com/nvim-neorocks/lux/pull/597))

## `lux-cli` - [0.3.5](https://github.com/nvim-neorocks/lux/compare/v0.3.4...v0.3.5) - 2025-04-14

### Fixed
- *(cli/install-rockspec)* ensure luarocks is installed
- *(build)* wrap binaries ([#583](https://github.com/nvim-neorocks/lux/pull/583))

## `lux-lib` - [0.3.5](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.3.4...lux-lib-v0.3.5) - 2025-04-14

### Added
- better dev version parsing
- better variable expansion + error on missing variables

### Fixed
- install pre-packaged luarocks on windows ([#584](https://github.com/nvim-neorocks/lux/pull/584))
- *(build)* wrap binaries ([#583](https://github.com/nvim-neorocks/lux/pull/583))

### Other
- *(deps)* bump bon from 3.5.0 to 3.6.0 ([#586](https://github.com/nvim-neorocks/lux/pull/586))

## `lux-cli` - [0.3.4](https://github.com/nvim-neorocks/lux/compare/v0.3.3...v0.3.4) - 2025-04-13

### Other
- updated the following local packages: lux-lib
## `lux-cli` - [0.3.3](https://github.com/nvim-neorocks/lux/compare/v0.3.2...v0.3.3) - 2025-04-11

### Other
- updated the following local packages: lux-lib
## `lux-cli` - [0.3.2](https://github.com/nvim-neorocks/lux/compare/v0.3.1...v0.3.2) - 2025-04-10

### Other
- updated the following local packages: lux-lib
## `lux-cli` - [0.3.1](https://github.com/nvim-neorocks/lux/compare/v0.3.0...v0.3.1) - 2025-04-10

### Other
- update Cargo.lock dependencies
## `lux-lib` - [0.3.1](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.3.0...lux-lib-v0.3.1) - 2025-04-10

### Fixed
- `[run]` field overwritten by `extra.rockspec` ([#566](https://github.com/nvim-neorocks/lux/pull/566))
- unsupported off-spec `install.bin` array field

## `lux-cli` - [0.3.0](https://github.com/nvim-neorocks/lux/compare/v0.2.4...v0.3.0) - 2025-04-08

### Added
- *(debug project)* flag to list included files ([#556](https://github.com/nvim-neorocks/lux/pull/556))

### Fixed
- [**breaking**] incompatible generated rockspec dependencies

### Other
- make `lx debug`'s description more obvious

## `lux-lib` - [0.3.0](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.2.3...lux-lib-v0.3.0) - 2025-04-08

### Added
- *(debug project)* flag to list included files ([#556](https://github.com/nvim-neorocks/lux/pull/556))

### Fixed
- *(build)* properly handle legacy rockspecs ([#557](https://github.com/nvim-neorocks/lux/pull/557))
- [**breaking**] incompatible generated rockspec dependencies

## `lux-cli` - [0.2.4](https://github.com/nvim-neorocks/lux/compare/v0.2.3...v0.2.4) - 2025-04-08

### Fixed
- *(help)* remove [UNIMPLEMENTED] from `lx doc` help

## `lux-lib` - [0.2.3](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.2.2...lux-lib-v0.2.3) - 2025-04-08

### Fixed
- *(rockspec)* support undocumented string/array duality

## `lux-cli` - [0.2.3](https://github.com/nvim-neorocks/lux/compare/v0.2.2...v0.2.3) - 2025-04-07

### Added
- *(build)* flag to build only dependencies

### Fixed
- fix!(sync): lock constraint changes when syncing with project lockfile
- *(build)* project not added to lockfile

## `lux-lib` - [0.2.2](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.2.1...lux-lib-v0.2.2) - 2025-04-07

### Fixed
- fix!(sync): lock constraint changes when syncing with project lockfile

## `lux-cli` - [0.2.2](https://github.com/nvim-neorocks/lux/compare/v0.2.1...v0.2.2) - 2025-04-07

### Other
- updated the following local packages: lux-lib

## `lux-cli` - [0.2.1](https://github.com/nvim-neorocks/lux/compare/lux-cli-v0.2.0...lux-cli-v0.2.1) - 2025-04-06

### Other
- add `repository` for `lux-cli` so that `cargo binstall` works

## `lux-cli` - [0.2.0](https://github.com/nvim-neorocks/lux/compare/lux-cli-v0.1.0...lux-cli-v0.2.0) - 2025-04-06

### Added
- implicitly propagate environment variables to subprocesses
- enable vim mode for `lx new` selections
- `lx run` command
- *(`lx new`)* create `src` directory automatically
- *(pin)* operate on lux.toml if in a project ([#486](https://github.com/nvim-neorocks/lux/pull/486))
- build project on `lx lua` ([#485](https://github.com/nvim-neorocks/lux/pull/485))
- [**breaking**] allow overriding `etc` tree ([#457](https://github.com/nvim-neorocks/lux/pull/457))
- feat!(toml): `opt` and `pin` fields ([#456](https://github.com/nvim-neorocks/lux/pull/456))
- [**breaking**] optional packages ([#453](https://github.com/nvim-neorocks/lux/pull/453))
- `lux.loader`
- compute hashes for rockspecs dynamically
- *(update)* `--toml` flag to upgrade packages in lux.toml ([#449](https://github.com/nvim-neorocks/lux/pull/449))
- *(remove)* operate on projects ([#448](https://github.com/nvim-neorocks/lux/pull/448))
- *(update)* take an optional list of packages ([#446](https://github.com/nvim-neorocks/lux/pull/446))
- feat!(cli): remove `sync` command
- *(update)* operate on lux.toml and lux.lock if in a project ([#428](https://github.com/nvim-neorocks/lux/pull/428))

### Fixed
- use compilation target to get platform identifier ([#512](https://github.com/nvim-neorocks/lux/pull/512))
- `lx run` does not rebuild the project
- *(`lx new`)* don't search parents for existing project ([#493](https://github.com/nvim-neorocks/lux/pull/493))
- `no such file or directory` when running `lx fmt`
- *(uninstall)* properly handle dependencies

### Other
- turn `run_lua` into an operation
- [**breaking**] rename `lx run` to `lx exec`
- *(deps)* bump octocrab from 0.43.0 to 0.44.0 ([#499](https://github.com/nvim-neorocks/lux/pull/499))
- *(build)* add case for local project with no source ([#490](https://github.com/nvim-neorocks/lux/pull/490))
- inconsistent naming in `lx debug project`
- refactor!(toml): extract `LuaDependency` type ([#454](https://github.com/nvim-neorocks/lux/pull/454))
- prepare flake for new build sequence
- *(deps)* bump tokio from 1.43.0 to 1.44.0 ([#461](https://github.com/nvim-neorocks/lux/pull/461))
- [**breaking**] introduce `LocalLuaRockspec` and `RemoteLuaRockspec`
- [**breaking**] allow building of local rockspecs
- [**breaking**] break apart `ProjectToml` into `LocalProjectToml` and `RemoteProjectToml`
- [**breaking**] break rockspec apart into `LocalRockspec` and `RemoteRockspec`

## `lux-lib` - [0.2.0](https://github.com/nvim-neorocks/lux/compare/lux-lib-v0.1.0...lux-lib-v0.2.0) - 2025-04-06

### Added
- implicitly propagate environment variables to subprocesses
- `lx run` command
- add `operations::run`
- *(`lux.toml`)* add `[run]` support
- *(pin)* operate on lux.toml if in a project ([#486](https://github.com/nvim-neorocks/lux/pull/486))
- *(build)* respect ignore files when copying source ([#495](https://github.com/nvim-neorocks/lux/pull/495))
- [**breaking**] allow overriding `etc` tree ([#457](https://github.com/nvim-neorocks/lux/pull/457))
- feat!(toml): `opt` and `pin` fields ([#456](https://github.com/nvim-neorocks/lux/pull/456))
- [**breaking**] optional packages ([#453](https://github.com/nvim-neorocks/lux/pull/453))
- `lux.loader`
- Lua API
- *(build)* treesitter-parser build backend ([#452](https://github.com/nvim-neorocks/lux/pull/452))
- compute hashes for rockspecs dynamically
- *(update)* `--toml` flag to upgrade packages in lux.toml ([#449](https://github.com/nvim-neorocks/lux/pull/449))
- *(remove)* operate on projects ([#448](https://github.com/nvim-neorocks/lux/pull/448))
- *(update)* take an optional list of packages ([#446](https://github.com/nvim-neorocks/lux/pull/446))
- allow `--tree` to override project tree ([#432](https://github.com/nvim-neorocks/lux/pull/432))
- *(update)* operate on lux.toml and lux.lock if in a project ([#428](https://github.com/nvim-neorocks/lux/pull/428))

### Fixed
- use compilation target to get platform identifier ([#512](https://github.com/nvim-neorocks/lux/pull/512))
- do not include `lua` as part of dependencies in TOML rockspecs
- map between luarocks and semver versions ([#483](https://github.com/nvim-neorocks/lux/pull/483))
- *(build)* don't fall back to `.src.rock` for local sources ([#494](https://github.com/nvim-neorocks/lux/pull/494))
- *(`lx new`)* don't search parents for existing project ([#493](https://github.com/nvim-neorocks/lux/pull/493))
- disallow `lua` in `dependencies` field
- *(uninstall)* properly handle dependencies
- *(build)* copy_directories into etc subdirectories ([#462](https://github.com/nvim-neorocks/lux/pull/462))
- minimize extraneous compiler output

### Other
- *(deps)* bump zip from 2.5.0 to 2.6.0 ([#514](https://github.com/nvim-neorocks/lux/pull/514))
- turn `run_lua` into an operation
- [**breaking**] rename `lx run` to `lx exec`
- *(deps)* bump zip from 2.4.1 to 2.5.0 ([#492](https://github.com/nvim-neorocks/lux/pull/492))
- *(deps)* bump zip from 2.3.0 to 2.4.1
- refactor!(toml): extract `LuaDependency` type ([#454](https://github.com/nvim-neorocks/lux/pull/454))
- *(lockfile)* hide unnecessarily public structs/methods
- [**breaking**] remove `lua` cargo feature
- *(nix)* fix `nix flake check`
- prepare flake for new build sequence
- [**breaking**] name all lockfiles `lux.lock`
- *(deps)* bump zip from 2.2.0 to 2.3.0 ([#470](https://github.com/nvim-neorocks/lux/pull/470))
- *(deps)* bump bon from 3.4.0 to 3.5.0 ([#469](https://github.com/nvim-neorocks/lux/pull/469))
- *(deps)* bump tokio from 1.43.0 to 1.44.0 ([#461](https://github.com/nvim-neorocks/lux/pull/461))
- *(deps)* bump bon from 3.3.2 to 3.4.0 ([#455](https://github.com/nvim-neorocks/lux/pull/455))
- [**breaking**] introduce `LocalLuaRockspec` and `RemoteLuaRockspec`
- *(`rocks pack`)* [**breaking**] disallow paths to `lux.toml` files
- [**breaking**] allow building of local rockspecs
- [**breaking**] break apart `ProjectToml` into `LocalProjectToml` and `RemoteProjectToml`
- [**breaking**] break rockspec apart into `LocalRockspec` and `RemoteRockspec`
- use crane for clippy and rustfmt checks ([#450](https://github.com/nvim-neorocks/lux/pull/450))
- *(deps)* bump flate2 from 1.0.35 to 1.1.0 ([#438](https://github.com/nvim-neorocks/lux/pull/438))
