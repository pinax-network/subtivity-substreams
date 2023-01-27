#[path = "pb/subtivity.v1.rs"]
#[allow(dead_code)]
pub mod subtivity;
pub use self::subtivity::*;

pub mod keyer;
pub use self::keyer::*;

use substreams::{prelude::*};
use prost_types::Timestamp;

pub fn get_counter(timestamp: Timestamp, store: StoreGetInt64, name: &str, interval: i64) -> Option<Counter> {
    // keyes
    let seconds = keyer::get_rem_euclid(timestamp.seconds - interval, interval); // -1 interval to get the previous interval
    let key = keyer::get_key(seconds, interval);
    
    // skip blocks
    if timestamp.nanos > 0 { return None } // skip blocks with partial timestamps
    if timestamp.seconds - interval != seconds { return None } // only return the last block of the interval

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

pub fn get_counters(timestamp: Timestamp, store_traces_count: StoreGetInt64, store_action_count: StoreGetInt64) -> Option<Counters> {
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
    let interval = keyer::DAY;
    let action_count = get_counter(timestamp.clone(), store_action_count, "action_count", interval);
    let traces_count = get_counter(timestamp.clone(), store_traces_count, "traces_count", interval);
    if action_count.is_some() { counters.push(action_count.unwrap()) }
    if traces_count.is_some() { counters.push(traces_count.unwrap()) }
    if counters.len() == 0 { return None }

    Some(Counters{counters})
}
