use substreams::{errors::Error, pb::substreams::Clock, log};
use substreams_sink_prometheus::{PrometheusOperations, Counter};
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;

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

#[substreams::handlers::map]
pub fn kv_out(stats: BlockStats, clock: Clock) -> Result<KvOperations, Error> {
    let mut kv_out = KvOperations::default();
    let seconds = clock.timestamp.unwrap().seconds;
    let epoch = (seconds / 86400) * 86400;

    let day = epoch / 86400;
    let value: u8 = 1;

    log::debug!("inside the kv_out function:");

    for wallet in stats.uaw.iter() {
        let key = format!("daw:{}:{}", day, wallet);
        kv_out.push_new(key, &[value], 1);
    }    
    Ok(kv_out)
}