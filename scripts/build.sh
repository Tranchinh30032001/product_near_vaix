#!/bin/bash
set -e
cd "`dirname $0`"/../ft
cargo build --all --target wasm32-unknown-unknown --release
cd ..
cd "`dirname $0`"/../action-hub
cargo build --all --target wasm32-unknown-unknown --release
cd ..
cp ./target/wasm32-unknown-unknown/release/*.wasm ./res/