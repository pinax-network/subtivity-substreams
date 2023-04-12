use substreams::errors::Error;
use substreams_sink_prometheus::{PrometheusOperations, Counter};

use crate::pb::BlockStats;

#[substreams::handlers::map]
pub fn prom_out(stats: BlockStats) -> Result<PrometheusOperations, Error> {
    let mut prom_out = PrometheusOperations::default();

    if stats.trace_calls > 0 {
        prom_out.push(Counter::from("trace_calls").add(stats.trace_calls as f64));
    }

    if stats.transaction_traces > 0 {
        prom_out.push(Counter::from("transaction_traces").add(stats.transaction_traces as f64));
    }

    Ok(prom_out)
}
