# **Subtivity** Block for `Antelope`

### Quickstart

```
$ substreams run -e eos.firehose.eosnation.io:9001 map_block -s 290000000 -t +1
```

### Graph

```mermaid
graph TD;
  map_block[map: map_block]
  sf.antelope.type.v2.Block[source: sf.antelope.type.v2.Block] --> map_block
```

### Modules

```yaml
Package name: subtivity_block_antelope
Version: v0.1.0
Doc: Subtivity Block for Antelope
Modules:
----
Name: map_block
Initial block: 0
Kind: map
Output Type: proto:subtivity.v1.BlockStats
Hash: d8df97e41aea26acde6b5e891f4ba75cba71fada
```