CREATE TABLE IF NOT EXISTS BlockStats  (
    transaction_traces Int64,
    trace_calls  Int64,
    uaw  Array(String),
    blobs  Int64
)
ENGINE = ReplacingMergeTree()
-- primary key = clock.id --
PRIMARY KEY (id)
ORDER BY (id)
