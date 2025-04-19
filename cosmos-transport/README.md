# cosmos-transport

`cosmos-transport` is the network access layer for the Cosmos ecosystem. It provides clean, modular functionality for fetching remote content—without expanding Cosmos’ trust model or introducing hidden complexity.

> Designed for auditable, minimal systems. No background syncs. No magic. Just fetch.

---

## 🌐 Overview

`cosmos-transport` is a standalone crate dedicated to remote content fetching. It is responsible for:

- Downloading `.tar.gz` files during installations
- Syncing Galaxy metadata over HTTP
- Supporting extensible transport protocols like HTTPS, FTP, or IPFS

All downloads are assumed to come from trusted sources. The crate does **not** verify signatures or perform cryptographic validation—that logic lives elsewhere in Cosmos.

---

## ✅ Philosophy

- Transport should be **pluggable**, not baked in
- `cosmos-core` will no longer verify non-local URLs (`file://`), delegating all networked logic to `cosmos-transport`
- `cosmos-core` does not link to HTTP libraries directly
- This crate compiles by default with only basic HTTP support
- TLS and other advanced transports are opt-in

This isolation ensures predictability, modularity, and clear separation of concerns.

---

## 📦 Included Functionality

### Current API:

```rust
fetch_bytes(url: &str) -> Result<Vec<u8>, TransportError>
supports_url(url: &str) -> bool
```

- `fetch_bytes`: Downloads content from supported URLs and returns raw bytes.
- `supports_url`: Validates URL scheme support based on enabled feature flags.

### Supported Schemes:

- `http://` (via `ureq`, included by default)
- `https://` (via `ureq/tls`, opt-in)

### Planned Schemes (roadmap or in-development):

***Please Note***: All the protocols below will be optional and enabled only by feature flags

- IPFS (`ipfs://`)
- FTP (`ftp://`)
- Git (`git://`)
- Onion routing (`.onion` / `tor://`)

### Explicitly Out-of-Scope:

- `file://` or local paths (handled in `cosmos-core`)
- Caching or **local** mirror fallback
- Retry logic, redirects, or background sync

---

## 🔧 Feature Flags

```toml
[features]
default = ["http"]
http = ["ureq"]
tls = ["ureq/tls"]
```

- The default build supports plain HTTP only
- TLS support can be enabled via the `tls` feature
- Higher-level crates (e.g., `cosmos-core`) control this via `transport-https`

---

## 🌍 Design Goals

- **Minimalist**: Focused only on remote data fetching
- **Flexible**: Easily extensible via feature flags
- **Secure-by-context**: Trust boundaries are respected—transport assumes the caller has already validated the source

This crate empowers Cosmos to operate in constrained, embedded, or auditable environments with confidence.

---

## 🔄 Future Plans

- Mirror support with deterministic fallback logic
- Parallel and range-based downloads (opt-in)
- Custom protocols via trait-based plugin system

For now? Keep it simple. Fetch the bytes. Pass them up. Let the rest of Cosmos decide what they mean.

---

## 🚀 Usage

Add to your `Cargo.toml`:

```toml
cosmos-transport = "*"
```

Enable TLS if needed:

```toml
cosmos-transport = { version = "*", features = ["tls"] }
```

Then call:

```rust
let data = cosmos_transport::fetch_bytes("http://example.com/foo.tar.gz")?;
```

---

## 🔍 See Also

- [`cosmos-core`](https://github.com/your-org/cosmos-core): Core install logic
- [`cosmos-docs`](https://github.com/your-org/cosmos-docs): Full project documentation

