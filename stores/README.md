# **Subtivity** Stores

### Quickstart

```
$ substreams run map_stores -s 1000 -t +1 --production-mode
$ substreams run -e eos.firehose.eosnation.io:9001 substreams.antelope.yaml map_stores -s 1000 -t +1  --production-mode
```

### Graph

```mermaid
graph TD;
  map_block[map: map_block]
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_block
  store_traces_count[store: store_traces_count]
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> store_traces_count
  map_block --> store_traces_count
  store_action_count[store: store_action_count]
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> store_action_count
  map_block --> store_action_count
  map_stores[map: map_stores]
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> map_stores
  store_action_count --> map_stores
  store_traces_count --> map_stores
```

### Modules

```yaml
Package name: subtivity_stores_evm
Version: v0.1.0
Doc: Subtivity Stores for EVM.
Modules:
----
Name: map_block
Initial block: 0
Kind: map
Output Type: proto:subtivity.v1.BlockStats
Hash: c82ddbb26f660b194707471cecfe0a61f19d1813

Name: store_traces_count
Initial block: 0
Kind: store
Value Type: int64
Update Policy: UPDATE_POLICY_ADD
Hash: fb8970ca5049e766827902440a359b251ee65a3f

Name: store_action_count
Initial block: 0
Kind: store
Value Type: int64
Update Policy: UPDATE_POLICY_ADD
Hash: 3e6e1033854a642630aa78824166d3e5b991f7a0

Name: map_stores
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.v1.Clock
Hash: add7dc82fb19d99292a3b5a18f77207de894d763
```