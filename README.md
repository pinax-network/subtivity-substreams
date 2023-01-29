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

### Deploy [`Badger DB`](https://github.com/dgraph-io/badger)

1. [Installing `Badger`](https://github.com/dgraph-io/badger#installing)
2. Run the sink: `substreams-sink-kv run badger3://badger_data.db mainnet.eth.streamingfast.io:443 substreams.yaml kv_out`


### Graph

```mermaid
graph TD;
  map_block_stats[map: map_block_stats]
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_block_stats
  sf.antelope.type.v2.Block[source: sf.antelope.type.v2.Block] --> map_block_stats
  store_transaction_traces[store: store_transaction_traces]
  map_block_stats --> store_transaction_traces
  store_trace_calls[store: store_trace_calls]
  map_block_stats --> store_trace_calls
  map_counters[map: map_counters]
  store_trace_calls -- deltas --> map_counters
  store_transaction_traces -- deltas --> map_counters
  kv_out[map: kv_out]
  map_counters --> kv_out
  db_out[map: db_out]
  map_counters --> db_out
```

### Modules

```yaml
Package name: subtivity_ethereum
Version: v0.1.0
Doc: Subtivity for Ethereum
Modules:
----
Name: map_block_stats
Initial block: 0
Kind: map
Output Type: proto:subtivity.v1.BlockStats
Hash: 8cf876aff1c5c206d2e7c4dc2186fe614e6d6181

Name: store_transaction_traces
Initial block: 0
Kind: store
Value Type: int64
Update Policy: UPDATE_POLICY_ADD
Hash: a29093257ea8bc6c5b9f0c2b2489fc099d483c3f

Name: store_trace_calls
Initial block: 0
Kind: store
Value Type: int64
Update Policy: UPDATE_POLICY_ADD
Hash: 9e599d84a1fabc850cd8535362ebe760d7a3f1f3

Name: map_counters
Initial block: 0
Kind: map
Output Type: proto:subtivity.v1.Counters
Hash: 920c66d65a49abae7212105ebb336fb2ef9e6b3c

Name: kv_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.kv.v1.KVOperations
Hash: 5ea5cc13a518199eb95e49e4a100f653a4cf0ed6

Name: db_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.database.v1.DatabaseChanges
Hash: 41725425562deb9b1cc1f6de56c6d9320dfc5400
```
