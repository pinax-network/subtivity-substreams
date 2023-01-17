# `Subtivity` Substreams

> Block level activity per for each supported chains **powered by Pinax**.

## Data

- [x] Transaction Count
- [x] Action Count (Events)
- [ ] UAW (Unique Active Wallets)

## Chains

- [ ] Ethereum
- [ ] Polygon
- [ ] Binance Smart Chain
- [x] Antelope

### Deploy [`Hasura`](https://hasura.io)

1. Run `docker-compose up`
2. Check out the [postgres-sink](https://github.com/streamingfast/substreams-sink-postgres#setup) and then run 
`go install ./cmd/substreams-sink-postgres` from within that directory (requires a proper Go installation, see 
[here](https://github.com/EOS-Nation/substreams-antelope-core#go) for instructions)
3. Run the sink: `substreams-sink-postgres run "psql://app_user:password@127.0.0.1:5432/app_db?sslmode=disable" "eos.firehose.eosnation.io:9001" chains/antelope/substreams.yaml db_out`
4. Open the Hasura console on `localhost:8080/console` and add the database under "Data" using this url: `postgresql://app_user:password@db:5432/app_db?sslmode=disable` and track the `hourly_stats` table

### Deploy [`Badger DB`](https://github.com/dgraph-io/badger)

1. [Installing `Badger`](https://github.com/dgraph-io/badger#installing)
2. Run the sink: `substreams-sink-kv run "badger3://badger_data.db" eos.firehose.eosnation.io:9001 chains/antelope/substreams.yaml kv_out`
