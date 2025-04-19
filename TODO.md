# 🛠️ Cosmos TODO + Roadmap (a.k.a. “Things We Swore We’d Fix Later”)

> Cosmos is ***self-aware***. It knows it’s a mess, and it’s working on it.

---

## 🚧 Phase 2 Goals (v1.0.0-1.9.9)

### 🌐 Strict Mode
- [ ] Add `strict_mode: bool` to `Config`
- [ ] Enforce version match in `GalaxyMeta` when strict is enabled
- [ ] Optional: support `--strict` flag for CLI override

### ⭐ Additional Features
- [ ] Improve script runner to print better exit status/errors
- [ ] Logging improvements for all crates
- [ ] Add dynamic locations for `config.toml`
- [ ] More detailed Nova stdout/stderr
- [ ] More edge case and error handling
- [ ] Make `install()` in `nova` optional, in case users only want a `build()` script for `stellar`

### 🖥️ General Code Changes
- [ ] Refactor all code, especially in `cosmos-core`
- [ ] Add tests for all existing code
- [ ] Actually document the code
- [ ] Implement FFI support for `cosmos-core`

### 🔄 Flow Improvements
- [ ] Add `--no-cross-galaxy` flag to `cosmos install` to skip cross-galaxy dependency resolution
- [ ] Check the existence of all dependencies before installing any of them
- [ ] Add `--safe` that disables `run()` from `install.lua`

---

## 🚀 Phase 3 Goals (beyond)
See [Phase 3](https://github.com/cosmospkg/cosmos-docs/tree/main/docs/12-Phase-3.md) for details.

## 🧪 Testing & UX

- [ ] Better error messages for missing Galaxy tarballs
- [ ] Add `--dry-run` flag to `cosmos install` for testing installs
- [ ] Add `cosmos doctor` or `cosmos validate` command for checking config + cache health
- [ ] `cosmos sync --dry-run` and `--diff` support

---

## 🧼 Stretch Goals

- [ ] Add `cosmos verify` for file hash checks
- [ ] `cosmos freeze` lockfile format
- [ ] `stellar test` to simulate Star installs in temp dirs
- [X] Fully replace shell scripting with Nova-only model (Phase 3+)
- [ ] Add `record_uninstall()` to `installer.rs` for tracking uninstalls
- [ ] Grow `nova` with more commands (see [Nova Doc](https://github.com/cosmospkg/cosmos-docs/tree/main/docs/10-Nova.md))
- [ ] Add `install_script = ["..."]` syntax later as a Nova helper macro, basically convert to a `nova` script
- [ ] Add `galaxy` option for dependencies in `star.toml` to force a specific galaxy when installing dependencies
- [ ] Document how to write custom Galaxies and Nova scripts
- [ ] Docs sidebar or index for newcomers
- [ ] Tag known good constellation.toml sets for common systems

---

## 🤖 Ongoing Philosophy Checks

- [x] No TLS, no GPG, no crypto delusions
- [x] No automatic behavior without user intent
- [x] No magic Git clones
- [x] Offline-first always

---

## 🏁 Victory Condition

**Phase 4**: `43 6F 73 6D 6F 73 20 68 61 73 20 77 6F 6E`

Cosmos has won.
