#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/staking_pool_factory.wasm ./res/
wasm-opt -Oz --signext-lowering ./res/staking_pool_factory.wasm -o ./res/staking_pool_factory.wasm

