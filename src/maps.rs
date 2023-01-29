use crate::subtivity::{Counter, Counters};
use substreams::errors::Error;
use substreams::{log, prelude::*};

#[substreams::handlers::map]
pub fn map_counters(store_traces_count: Deltas<DeltaInt64>, store_action_count: Deltas<DeltaInt64>) -> Result<Counters, Error> {
    let mut counters = Vec::new();

    for delta in store_traces_count.deltas {
        log::debug!("traces_count delta={:?}", delta);

        let key = format!("trace_count:{}", delta.key);
        let value = delta.new_value;
        if value > 0 {
            counters.push(Counter { key, value })
        }
    }

    for delta in store_action_count.deltas {
        log::debug!("action_count delta={:?}", delta);

        let key = format!("trace_count:{}", delta.key);
        let value = delta.new_value;
        if value > 0 {
            counters.push(Counter { key, value })
        }
    }
    Ok(Counters { counters })
}
