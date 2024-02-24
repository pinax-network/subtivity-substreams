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
	substreams run -e bitcoin.substreams.pinax.network:443 map_block_stats -s 814922 -t +1 -o jsonl

.PHONY: gui
gui:
	substreams gui -e bitcoin.substreams.pinax.network:443 map_block_stats -s 814922 -t +1
	# substreams gui -e mainnet.btc.streamingfast.io:443 map_block_stats -s 814922 -t +1

.PHONY: getblock
getblock:
	bitcoin-cli getblock 0000000000000000000221ce7e43028c690783a79d19c16f16482a3dad2f741b 3 > block.json


