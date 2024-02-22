CREATE TABLE IF NOT EXISTS BlockStats  (
    transaction_traces Int64,
    trace_calls  Int64,
    uaw  Array(String)
    blobs  Int64,
)
ENGINE = ReplacingMergeTree()
-- primary key = trx_id + action_index --
PRIMARY KEY (id)
ORDER BY (id)
