# Contributing to Cosmos

Thanks for your interest in contributing to Cosmos! Whether you're reporting a bug, requesting a feature, or writing code, you're in the right place.

This repository is for the main Cosmos CLI and ecosystem crates. Documentation, design rationale, and contribution policies live in a separate docs repo. Start there for full details:

📚 [Contribution Guide](https://github.com/cosmospkg/cosmos-docs/blob/main/docs/20-Cosmos-Contribution.md)  
🛠️ [Crate Policy](https://github.com/cosmospkg/cosmos-docs/blob/main/docs/22-Crate-Policy.md)  
🪐 [Full Cosmos Docs](https://docs.cosmos-pkg.org)

---

## Quick Guidelines

- **Check existing issues first** before opening a new one.
- Use relevant labels (`core`, `nova`, `stellar`, etc.) to scope your issue.
- Bug reports should include **repro steps** and expected vs actual behavior.
- Feature requests should include **why it matters**—not just what it does.

---

## Setup & Dev Environment

See [BUILDING.md](./BUILDING.md) for how to compile Cosmos from source, target musl, and strip binaries.

We use Rust with a workspace of modular crates:
- `cosmos-core` – shared logic
- `nova` – scripting engine
- `stellar` – package builder
- `cosmos-cli` – command-line interface
- `cosmos-transport` – download layers
- `cosmos-universe` – package state tracking

If you’re changing behavior, open an issue first (or reply to an existing one) so we can talk through it before code gets written.

---

## Communication

Feel free to:
- Open issues for bugs, questions, or feedback
- Submit pull requests with clearly scoped changes
- Use labels like `RFC`, `good first issue`, and `cosmic cleanup` for guidance

---

Cosmos is still young. Be kind, be clear, and don’t take bugs personally—we’re building this for people who fix systems with tarballs and spite.

Thanks again for contributing 🚀
