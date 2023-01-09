#!/usr/bin/env bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm_breakout \
  --out-dir wasm/target \
  --target web target/wasm32-unknown-unknown/release/breakout.wasm