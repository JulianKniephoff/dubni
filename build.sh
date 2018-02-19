#!/bin/sh

cargo install cargo-web
rustup target add wasm32-unknown-unknown
cargo web deploy --target=wasm32-unknown-unknown --release --verbose
