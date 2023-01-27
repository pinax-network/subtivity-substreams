#!/bin/bash

cargo build --target wasm32-unknown-unknown --release
substreams pack ./substreams.yaml
substreams graph $(ls *.spkg)
substreams info $(ls *.spkg)