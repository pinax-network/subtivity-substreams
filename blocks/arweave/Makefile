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

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: run
run:
	substreams run -e mainnet.arweave.streamingfast.io:443 map_block_stats -s 1370000 -t +100 -o jsonl

.PHONY: gui
gui:
	# substreams gui -e arweave.substreams.pinax.network:443 map_block_stats -s 1370000 -t +100
	substreams gui -e mainnet.arweave.streamingfast.io:443 map_block_stats -s 1370000 -t +100
