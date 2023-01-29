use substreams::{prelude::*, log};
use substreams::pb::substreams::Clock;

use crate::keyer;
use crate::subtivity::BlockStats;

#[substreams::handlers::store]
fn store_traces_count(clock: Clock, stats: BlockStats, s: StoreAddInt64) {
    let seconds = clock.timestamp.unwrap().seconds;
    for interval in [keyer::INTERVAL] {
        log::debug!("seconds {}: interval: {} adding traces count {}", seconds, interval, stats.traces_count);
        s.add(1, keyer::get_key(seconds, interval), stats.traces_count);
    }
}

#[substreams::handlers::store]
fn store_action_count(clock: Clock, stats: BlockStats, s: StoreAddInt64) {
    let seconds = clock.timestamp.unwrap().seconds;
    for interval in [keyer::INTERVAL] {
        log::debug!("seconds {}: interval: {} adding action count {}", seconds, interval, stats.action_count);
        s.add(1, keyer::get_key(seconds, interval), stats.action_count);
    }
}
