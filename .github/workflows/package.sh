#! /bin/sh
set -eux

# Validate that our code is always in a publishable state.
cargo publish --dry-run