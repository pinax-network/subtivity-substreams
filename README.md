# `Subtivity` Substreams

[<img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/pinax-network/subtivity-substreams/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/pinax-network/subtivity-substreams/actions?query=branch%3Amain)

> Block level activity per for each supported chains **powered by Pinax**.

## Data

- [x] Transaction Count
- [x] Action Count (Events)
- [x] UAW (Unique Active Wallets)

## Chains

- [x] Ethereum
  - [x] Polygon
  - [x] Binance Smart Chain
  - [x] Goerli
  - [x] Sepolia
  - [x] Rinkeby
  - [x] Mumbai
- [x] Antelope
  - [x] EOS
  - [x] WAX
  - [x] Telos
- [x] Near
- [x] Starknet
- [ ] Aptos

## Map Outputs

### `map_block_stats`

```json
{
  "transactionTraces": "213",
  "traceCalls": "1093",
  "uaw": [
    "4239a4e3a00a5282b6df7c19bd16cbf761b2c21f",
    "b18ccf69940177f3ec62920ddb2a08ef7cb16e8f",
    "603288a144fabf14a6c9806e9baadc9dbc1e9fd6",
    "0555262d2f4889522c3d7c0762d3c92e2ce817d1",
    "dc7bda95b512f7b9feb17566b80fa6bca5bb1693",
    "5c3efbafc55565d66312235428daf4988a4e41dc",
    ...
  ]
}
```

### Quickstart

```
$ make
$ make run
$ make gui
```

### Graph

```mermaid
graph TD;
  map_block_stats[map: map_block_stats]
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_block_stats
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_block_stats
  sf.near.type.v1.Block[source: sf.near.type.v1.Block] --> map_block_stats
  zklend.starknet.type.v1.Block[source: zklend.starknet.type.v1.Block] --> map_block_stats
  graph_out[map: graph_out]
  map_block_stats --> graph_out
```

### Modules

```yaml
Package name: subtivity_ethereum
Version: v0.4.0
Doc: Subtivity for Ethereum
Modules:
----
Name: map_block_stats
Initial block: 0
Kind: map
Output Type: proto:subtivity.v1.BlockStats
Hash: 93725ab06a11557d2f157350311fb73d3ac7437e

Name: graph_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.entity.v1.EntityChanges
Hash: e7d70cf4655838fa71eb62869bb34356714da241
```
