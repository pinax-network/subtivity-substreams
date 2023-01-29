use substreams::pb::substreams::Clock;
use substreams::{log, prelude::*};

use crate::keyer;
use crate::subtivity::BlockStats;

#[substreams::handlers::store]
fn store_traces_count(clock: Clock, stats: BlockStats, s: StoreAddInt64) {
    let seconds = clock.timestamp.unwrap().seconds;
    log::debug!(
        "seconds {}: interval: {} adding traces count {}",
        seconds,
        keyer::INTERVAL,
        stats.traces_count
    );
    s.add(1, keyer::get_key(seconds, keyer::INTERVAL), stats.traces_count);
}

#[substreams::handlers::store]
fn store_action_count(clock: Clock, stats: BlockStats, s: StoreAddInt64) {
    let seconds = clock.timestamp.unwrap().seconds;
    log::debug!(
        "seconds {}: interval: {} adding action count {}",
        seconds,
        keyer::INTERVAL,
        stats.action_count
    );
    s.add(1, keyer::get_key(seconds, keyer::INTERVAL), stats.action_count);
}
