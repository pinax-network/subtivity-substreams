use substreams::errors::Error;

use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
// use substreams_sink_kv::pb::kv::KvOperations;

use subtivity_common::subtivity::*;

#[substreams::handlers::map]
pub fn db_out(map_counters: Counters) -> Result<DatabaseChanges, Error> {
    let mut database_changes: DatabaseChanges = Default::default();
    
    // push to SQL database
    for counter in map_counters.counters { 
        let key = format!("{}:{}", counter.interval, counter.name);

        database_changes.push_change(counter.name, key, 0, Operation::Create)
            .change("seconds", (0, counter.seconds))
            .change("interval", (0, counter.interval))
            .change("traces_count", (0, counter.count ));

    }
    
    Ok(database_changes)
}
