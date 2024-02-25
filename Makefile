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
	cd blocks/beacon; $(MAKE) --no-print-directory build
	cd blocks/bitcoin; $(MAKE) --no-print-directory build
	cd blocks/arweave; $(MAKE) --no-print-directory build

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: pack
pack:
	substreams pack
	substreams pack substreams.antelope.yaml
	substreams pack substreams.beacon.yaml
	substreams pack substreams.near.yaml
	substreams pack substreams.starknet.yaml
	substreams pack substreams.bitcoin.yaml
	substreams pack substreams.arweave.yaml

.PHONY: graph
graph:
	substreams graph
	substreams graph substreams.antelope.yaml
	substreams graph substreams.beacon.yaml
	substreams graph substreams.near.yaml
	substreams graph substreams.starknet.yaml
	substreams graph substreams.bitcoin.yaml
	substreams graph substreams.arweave.yaml

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run -e eth.substreams.pinax.network:443 graph_out -s -100 -o jsonl

.PHONY: gui
gui:
	substreams gui -e eth.substreams.pinax.network:443 graph_out -s -100
