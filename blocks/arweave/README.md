# **Subtivity** Block for `Arweave`

### Quickstart

```bash
$ make
$ make run
$ make gui
```

### Graph

```mermaid
graph TD;
  map_block_stats[map: map_block_stats];
  sf.arweave.type.v1.Block[source: sf.arweave.type.v1.Block] --> map_block_stats;
```

### Modules

```yaml
Package name: subtivity_block_stats_arweave
Version: v0.1.0
Doc: Subtivity Block stats for Arweave
Modules:
----
Name: map_block_stats
Initial block: 0
Kind: map
Output Type: proto:subtivity.v1.BlockStats
Hash: c266a67a8110c615dbab796547c3f2d36ed1c256
```