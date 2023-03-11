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

.PHONY: run_antelope
run_antelope:
	substreams run -e eos.firehose.eosnation.io:9001 substreams.antelope.yaml prom_out -s 50000 -t +100000 -o jsonl

.PHONY: gui
gui:
	substreams gui -e mainnet.eth.streamingfast.io:443 prom_out -s 50000 -t +100000

.PHONY: sink_eth
sink_eth:
	substreams-sink-prometheus run -e mainnet.eth.streamingfast.io:443 https://github.com/pinax-network/subtivity-substreams/releases/download/v0.2.0/subtivity-ethereum-v0.2.0.spkg -s 16640000 -p 9103

.PHONY: sink_eos
sink_eos:
	substreams-sink-prometheus run -e eos.firehose.eosnation.io:9001 https://github.com/pinax-network/subtivity-substreams/releases/download/v0.2.0/subtivity-antelope-v0.2.0.spkg -s 294810000 -p 9104

.PHONY: sink_wax
sink_wax:
	substreams-sink-prometheus run -e wax.firehose.eosnation.io:9001 https://github.com/pinax-network/subtivity-substreams/releases/download/v0.2.0/subtivity-antelope-v0.2.0.spkg -s 230260000 -p 9105
