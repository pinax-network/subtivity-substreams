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
	substreams run -e api-unstable.streamingfast.io:443 prom_out -s 50000 -t +100000 -o jsonl

.PHONY: run_antelope
run_antelope:
	substreams run -e eos.firehose.eosnation.io:9001 substreams.antelope.yaml prom_out -s 50000 -t +100000 -o jsonl

.PHONY: gui
gui:
	substreams gui -e api-unstable.streamingfast.io:443 prom_out -s 50000 -t +100000

.PHONY: sink
sink:
	substreams-sink-prometheus run