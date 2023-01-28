use substreams::{prelude::*, log};
use substreams::errors::Error;
use substreams::pb::substreams::Clock;

use crate::keyer;
use crate::subtivity::{Counters, Counter};

#[substreams::handlers::map]
pub fn map_counters(clock: Clock, store_traces_count: StoreGetInt64, store_action_count: StoreGetInt64) -> Result<Counters, Error> {
    let counters = get_counters(clock, store_traces_count, store_action_count);
    if counters.is_none() { return Ok(Default::default()) }
    Ok(counters.unwrap())
}

pub fn get_counter(clock: Clock, store: StoreGetInt64, name: &str, interval: i64) -> Option<Counter> {
    // keyes
    let seconds = keyer::get_rem_euclid(keyer::to_seconds(clock.clone()) - interval, interval); // -1 interval to get the previous interval
    let key = keyer::get_key(seconds, interval);
    
    log::debug!("get_counter::seconds: {} key: {} interval: {}", seconds, key, interval);

    // skip blocks
    // if to_nanos(clock) > 0 { return None } // skip blocks with partial timestamps
    // if seconds - interval != seconds { return None } // only return the last block of the interval

    // get count from store
    let count = store.get_at(1, &key);
    if count.is_none() { return None }

    Some(Counter{
        name: name.to_string(),
        seconds,
        interval,
        count: count.unwrap(),
    })
}

pub fn get_counters(clock: Clock, store_traces_count: StoreGetInt64, store_action_count: StoreGetInt64) -> Option<Counters> {
    let mut counters = Vec::new();

    // // ISSUE ^^^^^^^^^^^^^^^^^^ value moved here, in previous iteration of loop
    // for interval in keyer::INTERVALS {
    //     let action_count = get_counter(block.clone(), store_action_count, "action_count", interval);
    //     let traces_count = get_counter(block.clone(), store_traces_count, "traces_count", interval);
    
    //     if action_count.is_none() && traces_count.is_none() { return None }
    //     if action_count.is_some() { counters.push(action_count.unwrap()) }
    //     if traces_count.is_some() { counters.push(traces_count.unwrap()) }
    // }

    // Push counters
    let interval = keyer::MINUTE;
    if keyer::to_seconds(clock.clone()) % interval != 0 { return None }

    let action_count = get_counter(clock.clone(), store_action_count, "action_count", interval);
    let traces_count = get_counter(clock, store_traces_count, "traces_count", interval);

    if action_count.is_some() { counters.push(action_count.unwrap()) }
    if traces_count.is_some() { counters.push(traces_count.unwrap()) }
    if counters.len() == 0 { return None }

    Some(Counters{counters})
}
