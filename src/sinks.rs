use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_entity_change::tables::Tables;
use substreams_entity_change::pb::entity::EntityChanges;
use crate::pb::BlockStats;

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, stats: BlockStats) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let row =  tables.create_row("BlockStats", clock.id);
    row.set("uaw", stats.uaw);
    row.set("trace_calls", stats.trace_calls);
    row.set("transaction_traces", stats.transaction_traces);

    Ok(tables.to_entity_changes())
}
