#!/bin/bash
# This script is an example install script for cosmos
cargo clean
cargo build --release --workspace

sudo install -Dm755 target/release/cosmos-cli /usr/bin/cosmos
sudo install -Dm755 target/release/stellar /usr/bin/stellar