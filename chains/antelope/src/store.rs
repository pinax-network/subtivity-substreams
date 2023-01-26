use substreams::{prelude::*, log};
use substreams_antelope::{Block};
use super::keyer;

#[substreams::handlers::store]
fn store_chain_id(s: StoreSetString) {
    s.set(1, "chain_id".to_string(), &"EOS".to_string())
}

#[substreams::handlers::store]
fn store_traces_count(block: Block, s: StoreAddInt64) {
    let traces_count = block.transaction_traces_count() as i64;
    let seconds = keyer::to_seconds(block.clone());
    log::debug!("block {}: seconds {}: adding transaction traces count {}", block.number, seconds, traces_count);

    // // ISSUE ^^^^^^^^^^^^^^^^^^ value moved here, in previous iteration of loop
    // for interval in keyer::INTERVALS {
    //     s.add(1, keyer::get_key(seconds, interval), traces_count);
    // }

    // s.add(1, keyer::get_key(seconds, keyer::ZERO), traces_count);
    // s.add(1, keyer::get_key(seconds, keyer::SECOND), traces_count);
    // s.add(1, keyer::get_key(seconds, keyer::MINUTE), traces_count);
    // s.add(1, keyer::get_key(seconds, keyer::HOUR), traces_count);
    s.add(1, keyer::get_key(seconds, keyer::DAY), traces_count);
    // s.add(1, keyer::get_key(seconds, keyer::WEEK), traces_count);
}

#[substreams::handlers::store]
fn store_action_count(block: Block, s: StoreAddInt64) {
    let action_count = block.executed_total_action_count() as i64;
    let seconds = keyer::to_seconds(block.clone());
    log::debug!("block {}: seconds {}: adding executed total action count {}", block.number, seconds, action_count);

    // // ISSUE ^^^^^^^^^^^^^^^^^^ value moved here, in previous iteration of loop
    // for interval in keyer::INTERVALS {
    //     s.add(1, keyer::get_key(seconds, interval), action_count);
    // }

    // s.add(1, keyer::get_key(seconds, keyer::ZERO), action_count);
    // s.add(1, keyer::get_key(seconds, keyer::SECOND), action_count);
    // s.add(1, keyer::get_key(seconds, keyer::MINUTE), action_count);
    // s.add(1, keyer::get_key(seconds, keyer::HOUR), action_count);
    s.add(1, keyer::get_key(seconds, keyer::DAY), action_count);
    // s.add(1, keyer::get_key(seconds, keyer::WEEK), action_count);
}