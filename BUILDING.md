# Building Cosmos

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

- This links dynamically to `libm.so.6` and `libc.so.6`
- ✅ Works fine on all glibc-based Linux distros
- ❌ Not portable to Alpine (musl)

### ✅ Static Build (recommended for releases)
```bash
rustup target add x86_64-unknown-linux-musl
RUSTFLAGS="-C target-feature=+crt-static" \
cargo build --release --target x86_64-unknown-linux-musl
```

- ✅ No `libm`, `libc`, or `ld-linux`
- ✅ Ideal for initramfs, Docker, chroots
- ✅ Works with Alpine Linux and embedded systems
- Output will be in:
```
target/x86_64-unknown-linux-musl/release/cosmos-cli
```

---

## 🔍 Testing Build Purity

### Check for dynamic dependencies:
```bash
ldd target/release/cosmos-cli
```
If static:
```
not a dynamic executable
```

### Inspect symbols:
```bash
file target/release/cosmos-cli
```
Should show `statically linked` if built with musl.

---

## 🧼 Optional Strip
```bash
strip target/.../cosmos-cli
```
Can reduce binary size by ~30-40%.

---

## 🔔 Note on `libm.so.6`

If you're building for **glibc**, you will see:
```text
/usr/lib/libm.so.6 (compatibility version 6.0.0)
```

This is normal and expected — the math library is required for some crates (`flate2`, `miniz_oxide`, etc.).

To fully avoid it, use **musl + static linking** as shown above.

---

## 🧪 Dev Testing Nova

Nova scripts are vendored and statically linked. You do **not** need Lua installed to build Cosmos.

If you want to test Nova scripts independently:
```bash
cargo run --package cosmos-cli -- install zlib --root ./testroot
```

---

Cosmos builds fast, links clean, and works anywhere you need to boot from a stick. 💿
