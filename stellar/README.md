# 🌟 stellar

> The official package builder for Cosmos.

Stellar is a CLI tool for maintainers and packagers. It handles scaffolding, building, validating, and indexing Stars and Nebulae. It is not required for using Cosmos, but it's the fastest way to create working Galaxies.

---

## ✨ Features

- Scaffold new Stars with metadata and install script
- Build `.tar.gz` archives from `files/` + `install.lua`
- Run Nova install logic during package build
- Validate Star metadata and script safety
- Fetch sources for manual repackaging
- Initialize and index Galaxies

---

## 🛠️ Commands

| Command                   | Purpose                                      |
|---------------------------|----------------------------------------------|
| `new-star <name>`         | Scaffold a new Star package                  |
| `build-star <path>`       | Build `.tar.gz` from `files/` + metadata     |
| `fetch <path>`            | Download remote source archive               |
| `validate <path>`         | Verify Star metadata and script              |
| `galaxy-init <name>`      | Create an empty Galaxy repo structure        |
| `index-galaxy <path>`     | Auto-populate `meta.toml` entries            |
| `lint <path>`             | *(future)* Style and structure suggestions   |

---

## ⭐ Example Workflow

```bash
stellar new-star hello
# edit star.toml, install.lua, files/
stellar fetch ./hello       # optional source download
stellar build-star ./hello  # creates dist/hello-0.1.0.tar.gz
stellar validate ./hello/star.toml
# manually add to core-galaxy
```

---

## 📁 Star Layout

```txt
stars/
  hello/
    star.toml
    install.lua
    files/
      usr/
        bin/
```

---

## 🧠 Notes

- Nova is preferred, but `install.sh` is also supported
- Only one install script is allowed per Star
- `files/` defines the install contents
- Stellar does **not** publish — it builds and validates locally

---

## 🧪 Build & Test

```bash
cargo build -p stellar
cargo test -p stellar
```

---

## 🔗 Project
- [Main Cosmos Repo](https://github.com/cosmospkg/cosmos)
- [Documentation](https://docs.cosmos-pkg.org/09-Tooling/)

---

> Stellar builds the Stars. Galaxies don’t form without it.
