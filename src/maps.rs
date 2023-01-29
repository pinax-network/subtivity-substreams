use substreams::{prelude::*, log};
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use crate::subtivity::{Counters, Counter};

#[substreams::handlers::map]
pub fn map_counters(clock: Clock, store_traces_count: Deltas<DeltaInt64>, store_action_count: Deltas<DeltaInt64>) -> Result<Counters, Error> {
    let mut counters = Vec::new();

    for delta in store_traces_count.deltas {
        log::debug!("traces_count delta={:?} clock={:?}", delta, clock);
        counters.push(Counter{
            key: format!("trace_count:{}", delta.key),
            value: delta.new_value,
        })
    }

    for delta in store_action_count.deltas {
        log::debug!("action_count delta={:?} clock={:?}", delta, clock);
        counters.push(Counter{
            key: format!("action_count:{}", delta.key),
            value: delta.new_value,
        })
    }
    Ok(Counters{counters})
}
