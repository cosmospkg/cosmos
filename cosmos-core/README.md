# 🧠 cosmos-core

> Core logic for the Cosmos package manager.

This crate implements all of the install, uninstall, dependency resolution, and file tracking logic that powers the Cosmos CLI. It is designed to be portable, deterministic, and CLI-agnostic.

---

## 🧩 Responsibilities

- Install Stars (packages) into a target root directory
- Recursively resolve dependencies across Galaxies
- Manage package metadata and install script execution
- Track installed files via the `cosmos-universe` crate
- Provide clean API surfaces for `cosmos-cli` and external tools

---

## 🔍 Key Features

- SemVer-style dependency resolution
- Nova or shell install script execution
- Root path redirection (`--root`) for chroot environments
- Handles Stars, Nebulae, and Constellations
- Designed for use by CLI, Stellar, or custom UIs

---

## 📁 Layout Overview

- `config.rs` – Loads system config (`config.toml`)
- `constellation.rs` – Handles install presets (constellation files)
- `error.rs` – Shared error types
- `galaxy.rs` – Loads and verifies Galaxy structure
- `installer.rs` – Runs install flows and scripts
- `resolver.rs` – Galaxy search and dependency resolution
- `star.rs` – Star (package) representation
- `universe.rs` – Communication with `cosmos-universe` for file tracking

---

## 🚀 Used By

- [`cosmos-cli`](../cosmos-cli) – CLI interface
- [`stellar`](../stellar) – Package builder (indirectly)
- Tests and third-party tooling for packaging or validation

---

## 🧪 Test & Build

```bash
cargo test -p cosmos-core
```

```bash
cargo build -p cosmos-core
```

---

## 🔗 Project
- [Main Cosmos Repo](https://github.com/cosmospkg/cosmos)
- [Documentation](https://docs.cosmos-pkg.org)

---

> cosmos-core is the heart of the system. Everything else is just a voice or a wrapper.
