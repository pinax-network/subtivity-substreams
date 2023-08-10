.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: pack
pack:
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run -e mainnet.near.streamingfast.io:443 map_block_stats -s 98505873 -t +10 -o jsonl

.PHONY: gui
gui:
	substreams gui -e mainnet.near.streamingfast.io:443 map_block_stats -s 98505873 -t +10
