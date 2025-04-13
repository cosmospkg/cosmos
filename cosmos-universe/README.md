# 🌌 cosmos-universe

> System state tracking for the Cosmos package manager.

This crate manages the contents of `universe.toml`, which records all installed Stars and their associated file paths. It is designed to be data-only — it holds no logic, performs no I/O, and enforces no business rules.

---

## 📁 Responsibilities

- Define the `Universe` and `InstalledStar` data structures
- Provide serialization/deserialization via `serde`
- Track which files were installed by each Star
- Expose helper functions for mutation (e.g. `record_install()`)

---

## 🧼 Design Philosophy

- No side effects
- No file system access
- No `std::fs` or `PathBuf` handling
- Just structured, predictable, testable state

This crate acts like a schema — a lightweight way for the Cosmos system to remember what it did.

---

## ⭐ Used By

- [`cosmos-core`](../cosmos-core) – calls `record_install()`, reads/writes `universe.toml`
- [`cosmos-cli`](../cosmos-cli) – uses `cosmos-core` for install/uninstall logic

---

## 🔍 API Example

```rust
let mut universe = Universe::new();
let installed = InstalledStar {
name: star.name.clone(),
version: star.version.clone(),
files,
};
universe.installed.insert(star.name.clone(), installed);
```

---

## 🔧 Build & Test

```bash
cargo build -p cosmos-universe
cargo test -p cosmos-universe
```

---

## 🔗 Project
- [Main Cosmos Repo](https://github.com/cosmospkg/cosmos)
- [Documentation](https://docs.cosmos-pkg.org)

---

> cosmos-universe remembers everything you install. It’s like a package log, but polite.
