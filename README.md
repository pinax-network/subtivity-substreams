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

### Deploy KV Sink

For testing purposes, [BadgerDB](https://dgraph.io/docs/badger/) is already included in the Substreams KV Sink, no need for any database install requirements.

```bash
$ substreams-sink-kv run badger3://badger_data.db \
  mainnet.eth.streamingfast.io:443 \
  substreams.yaml \
  kv_out
```

## Endpoints

- **Prometheus metrics server**: `localhost:9102`
- **ConnectWeb GRPC server**: `localhost:8000` (expose as reverse proxy using Nginx)

## Production deployment

- replace `subtreams.yaml` => `*.spkg GitHub release URL`
- replace `badger3://badger_data.db` => `tikv://pd0,pd1,pd2:2379?prefix=namespace_prefix`

[More info here](https://substreams.streamingfast.io/developers-guide/substreams-sinks/substreams-sink-kv#sending-to-a-production-key-value-store)

## Nginx

ConnectWeb server will need to be exposed as a public gRPC endpoint.

```nginx
server {
      listen 443 http2;
      listen [::]:443 http2;
      location / {
            grpc_pass grpc://localhost:8000;
      }
}
```


## Key Format

- **Format**: `<chain_id>:<counter>:<interval>:<seconds>`
- **Example**: `eth:trace_calls:86400:1525132800`

## Value Format (bytes)

Decoding Base64 String

```js
> Buffer.from("Nzc4MTMz","base64").toString()
'778133'
```

## Query Data

```
$ grpcurl --plaintext -d '{"key":"eth:trace_calls:86400:1525132800"}' localhost:8000 sf.substreams.sink.kv.v1.Kv/Get
```

| Method        | Request    |
|---------------|------------|
| `Get`         | `{"key":"eth:trace_calls:86400:1525132800"}`
| `GetMany`     | `{"keys":["eth:trace_calls:86400:1525132800","eth:trace_calls:86400:1525046400"]}`
| `GetByPrefix` | `{"prefix":"eth:trace_calls:86400"}`
| `Scan`        | `{"limit":1000}`

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
