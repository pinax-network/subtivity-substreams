#!/bin/bash

substreams protogen
cargo build --target wasm32-unknown-unknown --release
substreams pack
substreams graph
substreams info