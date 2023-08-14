#[path = "../../../src/pb/subtivity.v1.rs"]
#[allow(dead_code)]
pub mod pb;
pub use self::pb::BlockStats;

use std::collections::HashSet;

use substreams::errors::Error;
use substreams_antelope::pb::Block;

#[substreams::handlers::map]
pub fn map_block_stats(block: Block) -> Result<BlockStats, Error> {
    let mut unique_authorizations = HashSet::new();

    for trx in block.clone().all_transaction_traces() {
        for trace in &trx.action_traces {
            let action = trace.action.as_ref().unwrap();
            for authorization in action.authorization.clone() {
                unique_authorizations.insert(authorization.actor.clone());
            }
        }
    }

    Ok(BlockStats {
        transaction_traces: block.transaction_traces_count() as i64,
        trace_calls: block.executed_total_action_count() as i64,
        uaw: unique_authorizations.into_iter().collect(),
    })
}
