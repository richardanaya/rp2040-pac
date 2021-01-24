#!/usr/bin/env bash
set -x
set -e

# NOTE: Last executed using Rust 1.49.0

cargo install https://github.com/rust-embedded/svd2rust
cargo install --version 0.7.0  form
rustup component add rustfmt

rm -rf src
mkdir src
svd2rust -i svd/rp2040.svd
form -i lib.rs -o src
rm lib.rs
cargo fmt
