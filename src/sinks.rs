use std::borrow::BorrowMut;
use std::collections::HashMap;
use substreams::errors::Error;
use substreams::store::StoreAddBigInt;
use substreams::{log, pb::substreams::Clock};
use substreams_sink_prometheus::{Counter, Gauge, PrometheusOperations};

use crate::pb::BlockStats;

#[substreams::handlers::map]
pub fn prom_out(stats: BlockStats, clock: Clock) -> Result<PrometheusOperations, Error> {
    let mut prom_out = PrometheusOperations::default();

    if stats.trace_calls > 0 {
        prom_out.push(Counter::from("trace_calls").add(stats.trace_calls as f64));
    }

    if stats.transaction_traces > 0 {
        prom_out.push(Counter::from("transaction_traces").add(stats.transaction_traces as f64));
    }

    // TO-DO
    // push the daily unique wallets to prometheus
    let seconds = clock.timestamp.unwrap().seconds;
    let epoch = (seconds / 86400) * 86400;

    let day: i64 = epoch / 86400;
    let value: u8 = 1;
    let mut labels: HashMap<String, String> = HashMap::new();

    for wallet in stats.uaw.iter() {
        let key = format!("daw:{}:{}", day, wallet);
        labels.insert(key, value.to_string());
    }

    let mut gauge = Gauge::from("daw").with(labels);
    prom_out.push(gauge.set(1.0));

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
