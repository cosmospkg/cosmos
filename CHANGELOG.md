# Changelog

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