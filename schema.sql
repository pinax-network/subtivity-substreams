CREATE TABLE IF NOT EXISTS BlockStats  (
    transaction_traces Int64,
    trace_calls  Int64,
    uaw  Array(String)
)
ENGINE = MergeTree()
ORDER BY (address)