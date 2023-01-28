#!/bin/bash

cargo build --target wasm32-unknown-unknown --release
substreams pack
substreams pack substreams.evm.yaml
substreams graph
substreams info
