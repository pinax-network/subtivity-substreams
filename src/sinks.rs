use substreams::prelude::*;
use substreams::errors::Error;
use substreams_sink_kv::pb::kv::KvOperations;

use crate::keyer;

#[substreams::handlers::map]
pub fn kv_out(params: String, store_transaction_traces: Deltas<DeltaInt64>, store_trace_calls: Deltas<DeltaInt64>) -> Result<KvOperations, Error> {
    let mut kv_ops: KvOperations = Default::default();
    let chain_id = params.as_str();

    for delta in store_transaction_traces.deltas {
        let value = delta.new_value;
        if value > 0 {
            kv_ops.push_new(keyer::transaction_traces_key(chain_id, &delta.key), value.to_string(), 1);
        }
    }

    for delta in store_trace_calls.deltas {
        let value = delta.new_value;
        if value > 0 {
            kv_ops.push_new(keyer::trace_calls_key(chain_id, &delta.key), value.to_string(), 1);
        }
    }
    Ok(kv_ops)
}