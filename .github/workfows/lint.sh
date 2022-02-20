#! /bin/sh
set -eux

rustup component add clippy
rustup component add rustfmt

# Run all of our project through the Clippy linter.
cargo clippy --all-features --all-targets

# Ensure all of our project's formatting maches our rustfmt configuration.
cargo fmt -- --check