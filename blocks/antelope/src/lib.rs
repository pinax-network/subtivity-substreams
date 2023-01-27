use substreams::errors::Error;
use subtivity_common::{BlockSubtivity};
use substreams_antelope::pb::antelope::{Block};

#[substreams::handlers::map]
pub fn map_block(block: Block) -> Result<BlockSubtivity, Error> {
    let timestamp = block.header.unwrap().timestamp;

    Ok(BlockSubtivity{
        chain_id: "antelope".to_string(),
        block_num: block.number as u64,
        timestamp,
        traces_count: 0,
        action_count: 0,
        uaw: Vec::new()
    })
}
