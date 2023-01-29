#[path = "pb/subtivity.v1.rs"]
#[allow(dead_code)]
pub mod pb;

// use substreams::log;

use pb::BlockStats;
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
pub fn map_block_stats(block: Block) -> Result<BlockStats, Error> {
    // log::info!("Block: {:?}", block);

    let mut trace_calls: i64 = 0;
    for trace in block.transaction_traces.clone() {
        trace_calls += trace.calls.len() as i64;
    }

    Ok(BlockStats {
        transaction_traces: block.transaction_traces.len() as i64,
        trace_calls,
        uaw: Vec::new(), // TO-DO
    })
}
