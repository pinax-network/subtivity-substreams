use substreams::{prelude::*, log};
use substreams::errors::Error;
use substreams_antelope::pb::antelope::{Block};
use subtivity_common::{BlockSubtivity, keyer, get_counters, Counters};
use prost_types::Timestamp;

#[substreams::handlers::map]
pub fn map_counters(block: Block, store_traces_count: StoreGetInt64, store_action_count: StoreGetInt64) -> Result<Counters, Error> {
    let timestamp = to_timestamp(block);
    let counters = get_counters(timestamp, store_traces_count, store_action_count);
    if counters.is_none() { return Ok(Default::default()) }
    Ok(counters.unwrap())
}

#[substreams::handlers::store]
fn store_traces_count(block: BlockSubtivity, s: StoreAddInt64) {
    let seconds = to_seconds(block.clone());
    let traces_count = block.transaction_traces_count() as i64;
    log::debug!("block {}: seconds {}: adding transaction traces count {}", block.number, seconds, traces_count);

    // // ISSUE ^^^^^^^^^^^^^^^^^^ value moved here, in previous iteration of loop
    // for interval in keyer::INTERVALS {
    //     s.add(1, keyer::get_key(seconds, interval), traces_count);
    // }

    let interval = keyer::DAY;
    s.add(1, keyer::get_key(seconds, interval), traces_count);
}

#[substreams::handlers::store]
fn store_action_count(block: BlockSubtivity, s: StoreAddInt64) {
    let seconds = to_seconds(block.clone());
    let action_count = block.executed_total_action_count() as i64;
    log::debug!("block {}: seconds {}: adding executed total action count {}", block.number, seconds, action_count);

    // // ISSUE ^^^^^^^^^^^^^^^^^^ value moved here, in previous iteration of loop
    // for interval in keyer::INTERVALS {
    //     s.add(1, keyer::get_key(seconds, interval), action_count);
    // }

    let interval = keyer::DAY;
    s.add(1, keyer::get_key(seconds, interval), action_count);
}

pub fn to_timestamp(block: Block) -> Timestamp {
    block.header.unwrap().timestamp.unwrap()
}

pub fn to_seconds(block: Block) -> i64 {
    to_timestamp(block).seconds
}
