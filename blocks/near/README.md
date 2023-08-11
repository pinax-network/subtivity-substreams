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
Hash: 057a66ecc6526469b85b1a51a4573f2eac728af2
```