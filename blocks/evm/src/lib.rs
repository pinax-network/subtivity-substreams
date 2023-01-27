use substreams::errors::Error;
use subtivity_common::{BlockSubtivity};
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
pub fn map_block(block: Block) -> Result<BlockSubtivity, Error> {
    let timestamp = block.header.unwrap().timestamp;

    Ok(BlockSubtivity{
        chain_id: "evm".to_string(),
        block_num: block.number,
        timestamp,
        traces_count: 0,
        action_count: 0,
        uaw: Vec::new()
    })
}
