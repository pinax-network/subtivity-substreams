use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_entity_change::tables::Tables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_sink_prometheus::{Counter, PrometheusOperations};
use crate::pb::BlockStats;

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, stats: BlockStats) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let row =  tables.create_row("BlockStats", clock.id);
    row.set("uaw", stats.uaw);
    row.set("trace_calls", stats.trace_calls);
    row.set("transaction_traces", stats.transaction_traces);
    row.set("blobs", stats.blobs);

    Ok(tables.to_entity_changes())
}

#[substreams::handlers::map]
pub fn prom_out(stats: BlockStats) -> Result<PrometheusOperations, Error> {
    let mut prom_out = PrometheusOperations::default();

    if stats.trace_calls > 0 {
        prom_out.push(Counter::from("trace_calls").add(stats.trace_calls as f64));
    }

    if stats.transaction_traces > 0 {
        prom_out.push(Counter::from("transaction_traces").add(stats.transaction_traces as f64));
    }

    if stats.uaw.len() > 0 {
        prom_out.push(Counter::from("uaw").add(stats.uaw.len() as f64));
    }

    if stats.blobs > 0 {
        prom_out.push(Counter::from("blobs").add(stats.blobs as f64));
    }

    Ok(prom_out)
}