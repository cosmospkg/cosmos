# Building Cosmos

> Want a prebuilt binary instead? Check out the [Latest Release](https://github.com/cosmospkg/cosmos/releases/latest)

Cosmos is a static, offline-first package manager written in Rust. You can build it on any platform that supports Rust, but we recommend targeting **musl** for true static binaries.

---

## 🛠 Dependencies

- Rust (latest stable) via rustup
- `build-essential` or equivalent toolchain
- `musl-tools` (for Linux static builds)
- `strip`, `file`, or `ldd` for testing output

---

## 🧪 Build Targets

### ✅ Local Build (default, glibc)
```bash
cargo build --release
```

- Links dynamically to glibc libraries (`libm.so.6`, `libc.so.6`, `libgcc`)
- ✅ Works on most mainstream Linux distros
- ❌ Not portable to Alpine (musl)

### ✅ Static Build (recommended for releases)
```bash
rustup target add x86_64-unknown-linux-musl
RUSTFLAGS="-C target-feature=+crt-static" \
cargo build --release --target x86_64-unknown-linux-musl
```

- ✅ No glibc (`libm`, `libc`, `libgcc`) or `ld-linux` dependency
- ✅ Works with Alpine Linux, initramfs, Docker, embedded systems
- - Output binary: `target/x86_64-unknown-linux-musl/release/cosmos-cli`

---

## 🔍 Verifying Static Build

### Check for dynamic dependencies:
```bash
ldd target/x86_64-unknown-linux-musl/release/cosmos-cli
```
Expect something like:
```text
/lib/ld-musl-x86_64.so.1 (0x7f...)  # musl loader only
```

### Inspect binary type:
```bash
file target/x86_64-unknown-linux-musl/release/cosmos-cli
```
Expect output:
```text
ELF 64-bit LSB pie executable, x86-64, statically linked
```

Note: musl-based binaries *do* show a dynamic loader path (`ld-musl-x86_64.so.1`), but are still statically linked. This is normal.

---

## 🧼 Optional: Strip Binary
```bash
strip target/x86_64-unknown-linux-musl/release/cosmos-cli
```
Reduces size by ~30-40% with no loss in functionality.

---

### Optional: Copy it into your path:
```bash
cp target/x86_64-unknown-linux-musl/release/cosmos-cli /usr/local/bin/cosmos
```

---

## 🔔 Note on `libm.so.6`

If you're building for **glibc**, you may see:
```text
/usr/lib/libm.so.6 (compatibility version 6.0.0)
```
This is expected — `libm` is pulled in by crates like `flate2` or `miniz_oxide`.
To avoid this, build with **musl** as shown above.

---

## 🧪 Dev Testing Nova

Nova scripts are statically linked into the Cosmos binary. Lua is not required at runtime.

To test Nova scripts during development:
```bash
cargo run --package cosmos-cli -- install zlib --root ./testroot
```

---

Cosmos builds fast, links clean, and works anywhere you need to boot from a stick. 📏

