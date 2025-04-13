# ✨ nova

> The embedded Lua scripting engine for Cosmos.

Nova is a restricted Lua 5.4 runtime used to define safe, portable install logic for Stars. It replaces traditional shell scripts with something more controlled, auditable, and cross-platform.

---

## 🔍 What Nova Is

- An embedded [`mlua`](https://github.com/khvzak/mlua)-powered scripting engine
- Sandboxed with no access to raw `os.execute`, `io`, or arbitrary system calls
- Bound only to the APIs needed for Cosmos package installs
- Used by `cosmos-core` during Star installation

---

## ⭐ Core Features

- Run `install.lua` scripts during package installation
- Provide safe API bindings:
    - `copy(from, to)`
    - `symlink(target, linkname)`
    - `mkdir(path)`
    - `chmod(path, mode)`
    - `exists(path)`
    - `run(command)` (scoped to install root)
- Enforce install root sandboxing
- Executes from inside the extracted package temp dir

---

## 💡 Example Script

```lua
function install()
  copy("bin/hello", "/usr/bin/hello")
  chmod("/usr/bin/hello", 0o755)
end
```

---

## 🚧 Limitations

- No `os.execute` or raw system access
- All paths are resolved relative to an install root
- Scripts run non-interactively; failures abort install

---

## 📁 Layout

- `lib.rs` – runtime initialization, error handling, and Lua context wiring
- Exposes a single entrypoint: `run_nova_script(path, extraction_root, install_root)`

---

## 🔧 Build & Test

```bash
cargo build -p nova
cargo test -p nova
```

---

## 🔗 Project
- [Main Cosmos Repo](https://github.com/cosmospkg/cosmos)
- [Nova Docs](https://docs.cosmos-pkg.org/nova)

---

> Nova is your package’s installer script — with fewer footguns and more stars.
