# Changelog

## ## [v1.0.1] – 2025-05-06
- Fixed bug in Stellar `build_star` command where it required a metapackage to build files

## [v1.0.0] – 2025-04-30
This is the first stable release of Cosmos.

No changes from `v1.0.0-rc3`. All major functionality is considered production-ready.

## [v1.0.0-rc3] – 2025-04-16

- Added **tarball-level checksums** for packages. These are stored in Galaxy repository metadata and are **optional** for repo maintainers.
- Added **file-level checksums** for internal package contents. These are stored in each `star.toml` and are **optional** for package authors. Validation occurs after extraction.
- Rewrote downloader logic to use transport capabilities instead of hardcoding protocol checks. Will gracefully error if `--offline` is set and no files are found.
- Dependency constraint `"*"` now means "first available package from any Galaxy" — useful for flexible installs.
- `stellar build-star` now creates both a new `star.toml` and a matching `.tar.gz` package. Both are output to the `dist/` directory.
- `stellar` now supports automatic checksum generation for **extracted files** (during `build-star`) and **tarballs** (during `index-galaxy`).
- All CLI commands now include help descriptions. Type `--help` with any command to see usage info.


## [v1.0.0-rc2] – 2025-04-15
- Transport layer updated to `v1.0.0`
- Transport layer modules separated by protocol
- Better error handling for missing files in the CLI
- Added help text for `cosmos-cli` and `stellar` commands
- Hopefully fix static linking issues on `musl` systems

## [v1.0.0-rc] – 2025-04-13
- Initial public release candidate
- Core package manager (Cosmos) functional
- Stellar (build & repo tool) added
- Nova scripting engine embedded (Lua 5.4)
- Offline install support (offline-first design)
- Modular transport layer added
- Fixed: `star.toml` wouldn’t download on metadata-only sync
- Fixed: Galaxy cache loading now downloads missing star metadata unless `--offline` is used
