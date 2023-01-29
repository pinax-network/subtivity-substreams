use crate::subtivity::{Counter, Counters};
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams::{log, prelude::*};

#[substreams::handlers::map]
pub fn map_counters(
    clock: Clock,
    store_traces_count: Deltas<DeltaInt64>,
    store_action_count: Deltas<DeltaInt64>,
) -> Result<Counters, Error> {
    let mut counters = Vec::new();

    for delta in store_traces_count.deltas {
        let key = format!("trace_count:{}", delta.key);
        let value = delta.new_value;

        log::debug!("traces_count delta={:?} clock={:?}", delta, clock);
        if value == 0 { continue; }
        counters.push(Counter { key, value })
    }

    for delta in store_action_count.deltas {
        let key = format!("trace_count:{}", delta.key);
        let value = delta.new_value;

        log::debug!("action_count delta={:?} clock={:?}", delta, clock);
        if value == 0 { continue; }
        counters.push(Counter { key, value })
    }
    Ok(Counters { counters })
}
