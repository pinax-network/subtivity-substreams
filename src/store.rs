use substreams::pb::substreams::Clock;
use substreams::store::{StoreAdd, StoreNew};
use substreams::{errors::Error, log, store::StoreAddInt64};

use crate::pb::BlockStats;

// TO-DO
// save to internal Substreams store

#[substreams::handlers::store]
pub fn store_daw(stats: BlockStats, store: StoreAddInt64) {
    // let seconds = clock.timestamp.unwrap().seconds;
    // let epoch = (seconds / 86400) * 86400;

    // let day: i64 = epoch / 86400;
    let value: i64 = 1;

    log::debug!("inside the store_daw function:");

    for wallet in stats.uaw.iter() {
        let key = format!("daw:{}", wallet);
        store.add(1, key, value);
    }
}
