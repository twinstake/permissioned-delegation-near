#!/bin/bash
set -e

cargo build --target wasm32-unknown-unknown --release --all-features
cp target/wasm32-unknown-unknown/release/staking_pool.wasm ./res/
