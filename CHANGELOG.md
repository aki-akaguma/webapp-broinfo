# Changelog: broinfo

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
* android `versionCode` and `versionName` into build.rs


## [0.1.7] (2026-01-17)
### Added
* `out_dir` into `Disoxus.toml`
* android icon

### Changed
* update crate: browserinfocm(0.1.12),dioxus(0.7.3)

## [0.1.6] (2026-01-13)
### Changed
* updated crate: browserinfocm(0.1.11)

## [0.1.5] (2026-01-03)
### Fixed
* some documents

## [0.1.4] (2026-01-02)
### Added
* `bicmid`

### Changed
* updated crate: browserinfocm(0.1.10)
* rename the repository name: `broinfo` to `webapp-broinfo`

## [0.1.3] (2025-12-27)
### Added
* `user`
* this `version`
* `patched` by using `patch-crate`
* favicon
* `base_path`

### Changed
* set `https://aki.omusubi.org/broinfo/` in `set_server_url()` on mobile
* depend dioxus to aki's github to fixed a bug of `base_path`

### Fixed
* chained ip address

## [0.1.2] (2025-12-16)
### Added
* `dark mode`
* `host information`

### Changed
* split `browserinfo` and `browserinfocm` crates

## [0.1.1] (2025-12-01)
### Added
* `memory`, `timezone` and `device information` into info page
* depend: `ua-parser`
* `next_backend` and forwarder.rs
* sql table: `Referrer`, `Logs`
* `time stamp` to sql records.
* `user agent` to be saved before `broinfo`
* information table on screen
* minify js

### Changed
* submodule: src/utils/core to core and copy to resource/regexes.yaml
* refactor database
* the value of `hash` column to `base64` encoded value
* `broinfo` save format: `json` to `toml`
* splited javascript code to `broinfo.js`

## [0.1.0] (2025-11-18)
### Added
* first commit

[Unreleased]: https://github.com/aki-akaguma/broinfo/compare/v0.1.7..HEAD
[0.1.7]: https://github.com/aki-akaguma/broinfo/compare/v0.1.6..v0.1.7
[0.1.6]: https://github.com/aki-akaguma/broinfo/compare/v0.1.5..v0.1.6
[0.1.5]: https://github.com/aki-akaguma/broinfo/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/aki-akaguma/broinfo/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/broinfo/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/broinfo/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/broinfo/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/broinfo/releases/tag/v0.1.0
