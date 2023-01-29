# `Subtivity` Substreams

[<img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/pinax-network/subtivity-substreams/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/pinax-network/subtivity-substreams/actions?query=branch%3Amain)

> Block level activity per for each supported chains **powered by Pinax**.

## Data

- [x] Transaction Count
- [x] Action Count (Events)
- [ ] UAW (Unique Active Wallets)

## Chains

- [x] Ethereum
- [x] Antelope
- [ ] Polygon
- [ ] Binance Smart Chain

### Quickstart

```
$ substreams run map_counters -t +200 -o jsonl
```

**Running no-ETH chains**

```
$ substreams run -e <ENDPOINT> substreams.<CHAIN>.yaml map_counters -t +200 -o jsonl
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
Package name: subtivity_evm
Version: v0.1.0
Doc: Subtivity for EVM.
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
Hash: 181662bfafde369fd8147195da65f7cf4be74ff9

Name: store_action_count
Initial block: 0
Kind: store
Value Type: int64
Update Policy: UPDATE_POLICY_ADD
Hash: 070d2784d49f97bd345472c766616c5c933b8e81

Name: map_stores
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.v1.Clock
Hash: e34fe4a9a2c2e93984109187b16b86b5a1b9c78f
```

### Deploy [`Badger DB`](https://github.com/dgraph-io/badger)

1. [Installing `Badger`](https://github.com/dgraph-io/badger#installing)
2. Run the sink: `substreams-sink-kv run badger3://badger_data.db mainnet.eth.streamingfast.io:443 substreams.yaml kv_out`
