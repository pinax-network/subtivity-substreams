use substreams::errors::Error;

use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use substreams_sink_kv::pb::kv::KvOperations;

use subtivity_common::{Counters};

#[substreams::handlers::map]
pub fn db_out(map_counters: Counters) -> Result<DatabaseChanges, Error> {
    let mut database_changes: DatabaseChanges = Default::default();
    let chain = "chain".to_string(); // TO REPLACE: when Substreams supports parameters

    // push to SQL database
    for counter in map_counters.counters { 
        let key = format!("{}:{}:{}:{}", chain.clone(), counter.name, counter.interval, counter.seconds);
        let name = counter.name.to_string();

        database_changes.push_change(counter.name, key, 0, Operation::Create)
            .change("name", (name.clone(), name.clone()))
            .change("chain", (chain.clone(), chain.clone()))
            .change("seconds", (0, counter.seconds))
            .change("interval", (0, counter.interval))
            .change("count", (0, counter.count ));
    }

    Ok(database_changes)
}

#[substreams::handlers::map]
pub fn kv_out(map_counters: Counters) -> Result<KvOperations, Error> {
    let mut kv_ops: KvOperations = Default::default();
    let chain = "chain".to_string(); // TO REPLACE: when Substreams supports parameters

    // push to KV database
    for counter in map_counters.counters { 
        let key = format!("{}:{}:{}:{}", chain, counter.name, counter.interval, counter.seconds);
        kv_ops.push_new(key, counter.count.to_string(), 1);
    }
    Ok(kv_ops)
}
