#!/bin/bash

substreams protogen
cargo build --target wasm32-unknown-unknown --release
substreams pack
substreams pack substreams.antelope.yaml

echo "## Modules (Ethereum)"
substreams graph
substreams info

echo "## Modules (Antelope)"
substreams graph substreams.antelope.yaml
substreams info substreams.antelope.yaml
