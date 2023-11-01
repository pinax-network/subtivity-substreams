use substreams::errors::Error;
use substreams_entity_change::tables::Tables;
use substreams_entity_change::pb::entity::EntityChanges;
use crate::pb::BlockStats;

#[substreams::handlers::map]
pub fn graph_out(stats: BlockStats) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let mut key = "".to_string();
    for wallet in stats.uaw.iter() {
        key = format!("daw:{}", wallet);
    }
    let row =  tables
    .create_row("BlockStats", key);
    if stats.trace_calls > 0 {
        row.set("trace_calls", stats.trace_calls);
    }
    else{
        row.set("trace_calls", 0);
    }

    if stats.transaction_traces > 0 {
        row.set("transaction_traces", stats.transaction_traces);
    }
    else{
        row.set("transaction_traces", 0);
    }

    Ok(tables.to_entity_changes())
}
