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
	cd blocks/near; $(MAKE) --no-print-directory build
	cd blocks/starknet; $(MAKE) --no-print-directory build

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: pack
pack:
	substreams pack
	substreams pack substreams.antelope.yaml
	substreams pack substreams.near.yaml
	substreams pack substreams.starknet.yaml

.PHONY: graph
graph:
	substreams graph
	substreams graph substreams.antelope.yaml
	substreams graph substreams.near.yaml
	substreams graph substreams.starknet.yaml

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run -e eth.substreams.pinax.network:9000 prom_out -s -100 -o jsonl

.PHONY: gui
gui:
	substreams gui -e eth.substreams.pinax.network:9000 prom_out -s -100
