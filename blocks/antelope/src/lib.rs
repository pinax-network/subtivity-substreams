#[path = "pb/subtivity.v1.rs"]
#[allow(dead_code)]
pub mod pb;

use substreams::errors::Error;
use substreams_antelope::pb::antelope::Block;
use pb::BlockStats;

#[substreams::handlers::map]
pub fn map_block(block: Block) -> Result<BlockStats, Error> {
    Ok(BlockStats{
        traces_count: block.transaction_traces_count() as i64,
        action_count: block.executed_total_action_count() as i64,
        uaw: Vec::new() // TO-DO
    })
}
