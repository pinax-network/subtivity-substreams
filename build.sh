#!/bin/bash

substreams protogen
cargo build --target wasm32-unknown-unknown --release
substreams pack
substreams pack substreams.antelope.yaml
substreams graph

echo "## Modules (Ethereum)"
substreams info

echo "## Modules (Antelope)"
substreams info substreams.antelope.yaml