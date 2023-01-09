#!/usr/bin/env bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo install basic-http-server