use substreams::prelude::*;
use substreams::errors::Error;
use substreams_sink_kv::pb::kv::KvOperations;

use crate::keyer;

#[substreams::handlers::map]
pub fn kv_out(store_transaction_traces: Deltas<DeltaInt64>, store_trace_calls: Deltas<DeltaInt64>) -> Result<KvOperations, Error> {
    let params = "chain_id"; // NOT YET IMPLEMENTED IN SUBSTREAMS
    let mut kv_ops: KvOperations = Default::default();

    for delta in store_transaction_traces.deltas {
        let value = delta.new_value;
        if value > 0 {
            kv_ops.push_new(keyer::transaction_traces_key(params, &delta.key), value.to_string(), 1);
        }
    }

    for delta in store_trace_calls.deltas {
        let value = delta.new_value;
        if value > 0 {
            kv_ops.push_new(keyer::trace_calls_key(params, &delta.key), value.to_string(), 1);
        }
    }
    Ok(kv_ops)
}