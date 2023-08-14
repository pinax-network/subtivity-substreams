#[path = "../../../src/pb/subtivity.v1.rs"]
#[allow(dead_code)]
pub mod pb;
pub use self::pb::BlockStats;

use std::collections::HashSet;

use substreams::Hex;
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
pub fn map_block_stats(block: Block) -> Result<BlockStats, Error> {
    let mut trace_calls: i64 = 0;
    let mut unique_froms = HashSet::new();

    for trace in block.transaction_traces.clone() {
        trace_calls += trace.calls.len() as i64;
        unique_froms.insert(Hex(&trace.from).to_string());
    }

    Ok(BlockStats {
        transaction_traces: block.transaction_traces.len() as i64,
        trace_calls,
        uaw: unique_froms.into_iter().collect(),
    })
}
