use crate::subtivity::{Counter, Counters};
use substreams::errors::Error;
use substreams::{log, prelude::*};

#[substreams::handlers::map]
pub fn map_counters(store_transaction_traces: Deltas<DeltaInt64>, store_trace_calls: Deltas<DeltaInt64>) -> Result<Counters, Error> {
    let mut counters = Vec::new();

    for delta in store_transaction_traces.deltas {
        log::debug!("transaction_traces delta={:?}", delta);

        let key = format!("transaction_traces:{}", delta.key);
        let value = delta.new_value;
        if value > 0 {
            counters.push(Counter { key, value })
        }
    }

    for delta in store_trace_calls.deltas {
        log::debug!("trace_calls delta={:?}", delta);

        let key = format!("trace_calls:{}", delta.key);
        let value = delta.new_value;
        if value > 0 {
            counters.push(Counter { key, value })
        }
    }
    Ok(Counters { counters })
}
