use substreams::errors::Error;

use crate::subtivity::Counters;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use substreams_sink_kv::pb::kv::KvOperations;

#[substreams::handlers::map]
pub fn db_out(map_counters: Counters) -> Result<DatabaseChanges, Error> {
    let params = "params"; // NOT YET IMPLEMENTED IN SUBSTREAMS
    let mut database_changes: DatabaseChanges = Default::default();

    // push to SQL database (Postgres)
    for counter in map_counters.counters {
        let key = &format!("{}:{}", params, counter.key);
        // ISSUE: Operations Create and Update are not supported
        // NOT YET IMPLEMENTED IN SINKS
        database_changes
            .push_change("counters", key, 0, Operation::Create)
            .change("value", (0, counter.value));
    }
    Ok(database_changes)
}

#[substreams::handlers::map]
pub fn kv_out(map_counters: Counters) -> Result<KvOperations, Error> {
    let params = "params"; // NOT YET IMPLEMENTED IN SUBSTREAMS
    let mut kv_ops: KvOperations = Default::default();

    // push to KV database (BadgerDB)
    for counter in map_counters.counters {
        let key = &format!("{}:{}", params, counter.key);
        kv_ops.push_new(key, counter.value.to_string(), 1);
    }
    Ok(kv_ops)
}
