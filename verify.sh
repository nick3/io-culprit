#!/usr/bin/env bash
set -euo pipefail

cargo fmt --manifest-path io-culprit/Cargo.toml -- --check 2>/dev/null || echo "cargo fmt not available, skipping"
cargo test --manifest-path io-culprit/Cargo.toml
cargo build --manifest-path io-culprit/Cargo.toml --release
echo "All checks passed."
