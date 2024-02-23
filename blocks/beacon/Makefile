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
	substreams run map_block_stats -s 16500000 -t +100000 -o jsonl

.PHONY: gui
gui:
	substreams gui -e goerli-scdm37.mar.eosn.io:10016 map_block_stats -s 7671800 --plaintext
