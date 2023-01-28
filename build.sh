#!/bin/bash

substreams protogen
cargo build --target wasm32-unknown-unknown --release
substreams pack
substreams pack substreams.antelope.yaml
substreams graph
substreams info
