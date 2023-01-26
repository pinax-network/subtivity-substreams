use substreams::{prelude::*};
use substreams_antelope::{Block};

use super::pb::subtivity::{Counter, Counters};
use super::keyer;

pub fn get_counter(block: Block, store: StoreGetInt64, name: &str, interval: i64) -> Option<Counter> {
    // keyes
    let block_seconds = keyer::to_seconds(block.clone());
    let seconds = keyer::get_rem_euclid(block_seconds - interval, interval); // -1 interval to get the previous interval
    let key = keyer::get_key(seconds, interval);
    
    // skip blocks
    if keyer::to_nanos(block.clone()) > 0 { return None } // skip blocks with partial timestamps
    if block_seconds - interval != seconds { return None } // only return the last block of the interval

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

pub fn get_counters(block: Block, store_traces_count: StoreGetInt64, store_action_count: StoreGetInt64) -> Option<Counters> {
    let mut counters = Vec::new();

    // // ISSUE ^^^^^^^^^^^^^^^^^^ value moved here, in previous iteration of loop
    // for interval in keyer::INTERVALS {
    //     let action_count = get_counter(block.clone(), store_action_count, "action_count", interval);
    //     let traces_count = get_counter(block.clone(), store_traces_count, "traces_count", interval);
    
    //     if action_count.is_none() && traces_count.is_none() { return None }
    //     if action_count.is_some() { counters.push(action_count.unwrap()) }
    //     if traces_count.is_some() { counters.push(traces_count.unwrap()) }
    // }

    let interval = keyer::DAY;
    let action_count = get_counter(block.clone(), store_action_count, "action_count", interval);
    let traces_count = get_counter(block.clone(), store_traces_count, "traces_count", interval);

    if action_count.is_none() && traces_count.is_none() { return None }
    if action_count.is_some() { counters.push(action_count.unwrap()) }
    if traces_count.is_some() { counters.push(traces_count.unwrap()) }

    Some(Counters{counters})
}