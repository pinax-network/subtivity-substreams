# **Subtivity** Block for `Antelope`

### Quickstart

```
$ substreams run -e eos.firehose.eosnation.io:9001 substreams.yaml map_block -s 290000000 -t +10
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
Doc: Subtivity Block for Antelope.
Modules:
----
Name: map_block
Initial block: 0
Kind: map
Output Type: proto:subtivity.v1.BlockStats
Hash: e212d0a587f9e92b5de0b817f4168dafe2b66b50
```