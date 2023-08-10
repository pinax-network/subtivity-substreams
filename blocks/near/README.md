# **Subtivity** Block for `Near`

### Quickstart

```bash
$ make
$ make run
$ make gui
```

### Graph

```mermaid
graph TD;
  map_block_stats[map: map_block_stats]
  sf.near.type.v1.Block[source: sf.near.type.v1.Block] --> map_block_stats
```

### Modules

```yaml
Package name: subtivity_block_stats_near
Version: v0.1.0
Doc: Subtivity Block stats for Near
Modules:
----
Name: map_block_stats
Initial block: 0
Kind: map
Output Type: proto:subtivity.v1.BlockStats
Hash: 8cf876aff1c5c206d2e7c4dc2186fe614e6d6181
```