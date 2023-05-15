#[path = "../../../src/pb/subtivity.v1.rs"]
#[allow(dead_code)]
pub mod pb;
pub use self::pb::*;

use substreams::errors::Error;
use substreams_antelope::Block;

#[substreams::handlers::map]
pub fn map_block_stats(block: Block) -> Result<BlockStats, Error> {
    let mut uaw = Vec::new();

    for trx in block.clone().all_transaction_traces() {
        for trace in &trx.action_traces {
            let action = trace.action.as_ref().unwrap();
            if !uaw.contains(&action.account) {
                uaw.push(action.account.clone());
            }
        }
    }

    Ok(BlockStats {
        transaction_traces: block.transaction_traces_count() as i64,
        trace_calls: block.executed_total_action_count() as i64,
        uaw: uaw, // TO-DO
    })
}
