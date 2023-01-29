#[path = "pb/subtivity.v1.rs"]
#[allow(dead_code)]
pub mod pb;

use pb::BlockStats;
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
pub fn map_block_stats(block: Block) -> Result<BlockStats, Error> {
    Ok(BlockStats {
        traces_count: block.transaction_traces.len() as i64,
        action_count: 0, // TO-DO
        uaw: Vec::new(), // TO-DO
    })
}
