use substreams::errors::Error;
use substreams_sink_prometheus::{Counter, PrometheusOperations};

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

// TO-DO
// save to internal Substreams store

// #[substreams::handlers::store]
// pub fn store_daw(stats: BlockStats, clock: Clock, store: StoreAddBigInt) {
//     let seconds = clock.timestamp.unwrap().seconds;
//     let epoch = (seconds / 86400) * 86400;

//     let day: i64 = epoch / 86400;
//     let value: u8 = 1;

//     log::debug!("inside the store_daw function:");

//     for wallet in stats.uaw.iter() {
//         let key = format!("daw:{}:{}", day, wallet);
//     }
// }
