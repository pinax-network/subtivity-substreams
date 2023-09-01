.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

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
	substreams run -e starknet.substreams.pinax.network:9000 map_block_stats -s -10 -o jsonl

.PHONY: gui
gui:
	substreams gui -e starknet.substreams.pinax.network:9000 map_block_stats -s -10
