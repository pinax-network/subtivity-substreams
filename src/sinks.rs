// use std::collections::HashMap;

use substreams::errors::Error;
use substreams_sink_prometheus::{PrometheusOperations, Counter, Histogram, Summary};

use crate::pb::BlockStats;

#[substreams::handlers::map]
pub fn prom_out(stats: BlockStats) -> Result<PrometheusOperations, Error> {
    let mut prom_out = PrometheusOperations::default();

    if stats.trace_calls > 0 {
        let mut counter = Counter::from("trace_calls");
        let mut histogram = Histogram::from("trace_calls_histogram");
        let mut summary = Summary::from("trace_calls_histogram");
        prom_out.push(counter.add(stats.trace_calls as f64));
        prom_out.push(histogram.observe(stats.trace_calls as f64));
        prom_out.push(summary.observe(stats.trace_calls as f64));
    }

    if stats.transaction_traces > 0 {
        let mut counter = Counter::from("transaction_traces");
        let mut histogram = Histogram::from("transaction_traces_histogram");
        let mut summary = Summary::from("transaction_traces_histogram");
        prom_out.push(counter.add(stats.transaction_traces as f64));
        prom_out.push(histogram.observe(stats.transaction_traces as f64));
        prom_out.push(summary.observe(stats.transaction_traces as f64));
    }

    Ok(prom_out)
}
