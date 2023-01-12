# Pinax `Subtivity`

> Block level activity per for each supported chain **powered by Pinax**.

## Data

- [ ] Transaction Count
- [ ] Action Count (Events)
- [ ] UAW (Unique Active Wallets)

## Chains

- [ ] Ethereum
- [ ] Polygon
- [ ] Binance Smart Chain
- [x] Antelope

### Graph

```mermaid
graph TD;
  map_block_stats[map: map_block_stats]
  sf.antelope.type.v2.Block[source: sf.antelope.type.v2.Block] --> map_block_stats
  store_trx_count[store: store_trx_count]
  map_block_stats --> store_trx_count
  store_act_count[store: store_act_count]
  map_block_stats --> store_act_count
  store_max_trx_count[store: store_max_trx_count]
  map_block_stats --> store_max_trx_count
  store_max_action_count[store: store_max_action_count]
  map_block_stats --> store_max_action_count
  map_hourly_stats[map: map_hourly_stats]
  map_block_stats --> map_hourly_stats
  store_trx_count --> map_hourly_stats
  store_act_count --> map_hourly_stats
  db_out[map: db_out]
  map_block_stats --> db_out
  map_hourly_stats --> db_out
```

### Modules

```yaml
Name: map_block_stats
Initial block: 2
Kind: map
Output Type: proto:subtivity.v1.BlockStats
Hash: 2af0143fba6444485df438c6f3dc09bad6d5882f

Name: store_trx_count
Initial block: 2
Kind: store
Value Type: int64
Update Policy: UPDATE_POLICY_ADD
Hash: 57099541f52e4dc535bce18aca96c320e2a5cdff

Name: store_act_count
Initial block: 2
Kind: store
Value Type: int64
Update Policy: UPDATE_POLICY_ADD
Hash: e85e8a2fc370ad95c136cc37c7878d8846bb101d

Name: store_max_trx_count
Initial block: 2
Kind: store
Value Type: int64
Update Policy: UPDATE_POLICY_MAX
Hash: 2d8de7c6fcf7d3270638abbf2f6456f744b22fe7

Name: store_max_action_count
Initial block: 2
Kind: store
Value Type: int64
Update Policy: UPDATE_POLICY_MAX
Hash: 53944c0757c30bb9625f040087b6da3d24226a86

Name: map_hourly_stats
Initial block: 2
Kind: map
Output Type: proto:antelope.subtivity.v1.HourlyStats
Hash: dbc15000693b8845ac913fe3fc9a2b0689bb9ad9

Name: db_out
Initial block: 2
Kind: map
Output Type: proto:sf.substreams.database.v1.DatabaseChanges
Hash: e6c7b4af507003c3bcb05beeda21f50c3f542dac
```

### Protobuf

```protobuf
message HourlyStats {
  uint32 block_num = 1;                     // start block number of the accumulated interval
  google.protobuf.Timestamp timestamp = 2;  // block creation timestamp (UTC) of the start block
  int64 trx_count = 3;                      // number of successfully executed transactions in the interval
  int64 act_count = 4;                      // number of successfully executed actions in the interval
  string chain = 5;
}
```

### Deploy [`Hasura`](https://hasura.io)

1. Run `docker-compose up`
2. Check out the [postgres-sink](https://github.com/streamingfast/substreams-sink-postgres#setup) and then run 
`go install ./cmd/substreams-sink-postgres` from within that directory (requires a proper Go installation, see 
[here](https://github.com/EOS-Nation/substreams-antelope-core#go) for instructions)
3. Run the sink: `substreams-sink-postgres run "psql://app_user:password@127.0.0.1:5432/app_db?sslmode=disable" "eos.firehose.eosnation.io:9001" "substreams.yaml" db_out`
4. Open the Hasura console on `localhost:8080/console` and add the database under "Data" using this url: `postgresql://app_user:password@db:5432/app_db?sslmode=disable` and track the `hourly_stats` table