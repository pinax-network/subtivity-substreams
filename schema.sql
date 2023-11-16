CREATE TABLE IF NOT EXISTS BlockStats  (
    id String,
    transaction_traces Int64,
    trace_calls  Int64,
    uaw  Array(String)
)
ENGINE = ReplacingMergeTree()
PRIMARY KEY (id)
