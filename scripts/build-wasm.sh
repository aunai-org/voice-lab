#!/usr/bin/env bash
# Builds the voice-core WebAssembly wrapper into ./pkg (needs the wasm32 target and
# wasm-bindgen-cli 0.2.100: `rustup target add wasm32-unknown-unknown`,
# `cargo install wasm-bindgen-cli --version 0.2.100 --locked`).
set -euo pipefail
cd "$(dirname "$0")/.."
cargo build --manifest-path wasm/Cargo.toml --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir pkg --out-name voice_lab_wasm \
  wasm/target/wasm32-unknown-unknown/release/voice_lab_wasm.wasm
