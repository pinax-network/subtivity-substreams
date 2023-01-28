use substreams::errors::Error;
use subtivity_common::{BlockStats};
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
pub fn map_block(block: Block) -> Result<BlockStats, Error> {
    Ok(BlockStats{
        traces_count: block.transaction_traces.len() as i64,
        action_count: 0, // TO-DO
        uaw: Vec::new() // TO-DO
    })
}
