#[path = "pb/subtivity.v1.rs"]
#[allow(dead_code)]
pub mod subtivity;
pub use self::subtivity::*;

mod keyer;
use substreams::{prelude::*, log};
use substreams::errors::Error;
use substreams::pb::substreams::Clock;

use subtivity::BlockStats;

#[substreams::handlers::map]
pub fn map_stores(clock: Clock, _store_action_count: StoreGetInt64, _store_traces_count: StoreGetInt64 ) -> Result<Clock, Error> {
    Ok(clock)
}

#[substreams::handlers::store]
fn store_traces_count(clock: Clock, stats: BlockStats, s: StoreAddInt64) {
    let seconds = clock.timestamp.unwrap().seconds;
    for interval in keyer::INTERVALS {
        log::debug!("seconds {}: interval: {} adding traces count {}", seconds, interval, stats.traces_count);
        s.add(1, keyer::get_key(seconds, interval), stats.traces_count);
    }
}

#[substreams::handlers::store]
fn store_action_count(clock: Clock, stats: BlockStats, s: StoreAddInt64) {
    let seconds = clock.timestamp.unwrap().seconds;
    for interval in keyer::INTERVALS {
        log::debug!("seconds {}: interval: {} adding action count {}", seconds, interval, stats.action_count);
        s.add(1, keyer::get_key(seconds, interval), stats.action_count);
    }
}
