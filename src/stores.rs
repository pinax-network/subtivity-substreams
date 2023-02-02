use substreams::pb::substreams::Clock;
use substreams::{log, prelude::*};

use crate::keyer;
use crate::subtivity::BlockStats;

#[substreams::handlers::store]
fn store_transaction_traces(clock: Clock, stats: BlockStats, s: StoreAddInt64) {
    let seconds = clock.clone().timestamp.unwrap().seconds;
    if seconds > 0 {
        log::debug!("clock {:?} stats {:?}", clock, stats);
        s.add(1, keyer::get_key(seconds, keyer::INTERVAL), stats.transaction_traces);
    }
}

#[substreams::handlers::store]
fn store_trace_calls(clock: Clock, stats: BlockStats, s: StoreAddInt64) {
    let seconds = clock.clone().timestamp.unwrap().seconds;
    if seconds > 0 {
        log::debug!("clock {:?} stats {:?}", clock, stats);
        s.add(1, keyer::get_key(seconds, keyer::INTERVAL), stats.trace_calls);
    }
}
