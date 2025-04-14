# Changelog

## [v1.0.0-rc] – 2025-04-13
- Initial public release candidate
- Core package manager (Cosmos) functional
- Stellar (build & repo tool) added
- Nova scripting engine embedded (Lua 5.4)
- Offline install support (offline-first design)
- Modular transport layer added
- Fixed: `star.toml` wouldn’t download on metadata-only sync
- Fixed: Galaxy cache loading now downloads missing star metadata unless `--offline` is used