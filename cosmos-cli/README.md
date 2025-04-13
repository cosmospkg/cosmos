# 🚀 cosmos-cli

> The CLI interface for the Cosmos package manager.

This crate provides the end-user command-line tool `cosmos`, which is responsible for interacting with Cosmos Galaxies, installing and uninstalling Stars, and managing local package state.

---

## 🔧 Responsibilities

- Load and parse system `config.toml`
- Handle CLI flags and subcommands (via [`clap`](https://docs.rs/clap))
- Coordinate install/uninstall/update flows via `cosmos-core`
- Handle Galaxy syncing and caching logic
- Write changes to the local `universe.toml`

---

## ⭐ Features

- Install Stars (packages) from Galaxies (repos)
- Support for `--root` install target (e.g. for chroot environments)
- Support for `--offline` mode
- Support for Constellation installs (TOML-based presets)
- Minimal, reproducible, human-readable system state
- Launchable on recovery systems or from USB

---

## 💡 Usage Examples

```bash
# Install a Star
cosmos install hello

# Install to a mounted root
cosmos install --root /mnt/wombat core-stack

# Use offline mode (from cache only)
cosmos install --offline busybox

# Sync a remote Galaxy
cosmos sync --from http://example.com/galaxies/core

# Install from a Constellation file
cosmos install --constellation desktop.toml
```

---

## 🔧 Build

```bash
cargo build --release --bin cosmos-cli
```

To build a fully static binary with musl:
```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --bin cosmos-cli --target x86_64-unknown-linux-musl
strip ./target/x86_64-unknown-linux-musl/release/cosmos-cli
```

---

## 📁 Related Crates

- [`cosmos-core`](../cosmos-core) – core install/dependency logic
- [`cosmos-universe`](../cosmos-universe) – tracks installed package state
- [`nova`](../nova) – embedded Lua runtime for install scripts

---

## 🔗 Project
- [Main Cosmos Repo](https://github.com/cosmospkg/cosmos)
- [Documentation](https://docs.cosmos-pkg.org)

---

> cosmos-cli is the voice of Cosmos. All packages flow through here.
