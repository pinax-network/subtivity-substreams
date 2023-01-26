use substreams::{prelude::*};
use substreams::errors::Error;
use substreams_antelope::pb::antelope::{Block};

// use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
// use substreams_sink_kv::pb::kv::KvOperations;

mod keyer;
mod getter;
mod store;
mod pb;
use pb::subtivity::{Counters};

#[substreams::handlers::map]
pub fn map_counters(block: Block, store_traces_count: StoreGetInt64, store_action_count: StoreGetInt64) -> Result<Counters, Error> {
    let counters = getter::get_counters(block, store_traces_count, store_action_count);
    if counters.is_none() { return Ok(Default::default()) }
    Ok(counters.unwrap())
}

// #[substreams::handlers::map]
// pub fn db_out(map_counters: Counters) -> Result<DatabaseChanges, Error> {
//     let mut database_changes: DatabaseChanges = Default::default();
    
//     // // skip blocks
//     // if keyer::to_nanos(block) > 0 { return Ok(database_changes) } // skip blocks with partial timestamps

//     // let traces_count = store_traces_count.get_at(1, key);
//     // let action_count = store_action_count.get_at(1, key);

//     // // push to SQL database
//     // database_changes.push_change("traces", &key, 0, Operation::Create)
//     //     .change("seconds", (0, seconds))
//     //     .change("interval", (0, interval))
//     //     .change("traces_count", (0, store_traces_count.get_at(1, key) ))
//     //     .change("action_count", (0, store_action_count.get_at(1, key) ));
    
//     Ok(database_changes)
// }

// // #[substreams::handlers::map]
// // pub fn kv_out(map_counters: Counters) -> Result<KvOperations, Error> {

// //     let mut kv_ops: KvOperations = Default::default();

// //     for counter in map_counters.iter() {
// //         let key = keyer::get_key(counter.seconds, counter.interval);
// //         kv_ops.push_new(key, counter, 1);
// //     }
// //     Ok(kv_ops)
// // }
