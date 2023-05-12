.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	cd blocks/antelope; $(MAKE) --no-print-directory build
	cd blocks/ethereum; $(MAKE) --no-print-directory build

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: pack
pack:
	substreams pack
	substreams pack substreams.antelope.yaml

.PHONY: graph
graph:
	substreams graph
	substreams graph substreams.antelope.yaml

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run -e mainnet.eth.streamingfast.io:443 prom_out -s 50000 -t +100000 -o jsonl

.PHONY: gui
gui:
	substreams gui -e mainnet.eth.streamingfast.io:443 kv_out -s 50000 -t +100000
