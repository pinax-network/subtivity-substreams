use substreams::{prelude::*, log};
use substreams::errors::Error;
use substreams_ethereum::pb::ethereum::{Block}; 

#[substreams::handlers::map]
pub fn map_blocks( block: Block ) -> Result<Block, Error> {
    Ok(block)
}
