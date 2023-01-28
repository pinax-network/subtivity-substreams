use substreams::errors::Error;

use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use substreams_sink_kv::pb::kv::{KvOperations};
use crate::subtivity::{Counters};

#[substreams::handlers::map]
pub fn db_out(map_counters: Counters) -> Result<DatabaseChanges, Error> {
    let param = "param";
    let mut database_changes: DatabaseChanges = Default::default();

    // push to SQL database
    for counter in map_counters.counters { 
        let key = format!("{}:{}:{}:{}", param, counter.name, counter.interval, counter.seconds);
        // => param:traces_count:86400:1674864000

        database_changes.push_change(counter.name, key, 0, Operation::Create)
            .change("seconds", (0, counter.seconds))
            .change("interval", (0, counter.interval))
            .change("count", (0, counter.count ));
    }
    Ok(database_changes)
}

#[substreams::handlers::map]
pub fn kv_out(map_counters: Counters) -> Result<KvOperations, Error> {
    let param = "param";
    let mut kv_ops: KvOperations = Default::default();

    // push to SQL database
    for counter in map_counters.counters { 
        let key = format!("{}:{}:{}:{}", param, counter.name, counter.interval, counter.seconds);
        // => param:traces_count:86400:1674864000

        kv_ops.push_new(key, counter.count.to_string(), 1);
    }
    
    Ok(kv_ops)
}