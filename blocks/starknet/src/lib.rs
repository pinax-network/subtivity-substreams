#[path = "./pb/zklend.starknet.type.v1.rs"]
#[allow(dead_code)]
pub mod starknet;
pub use self::starknet::Block;

#[path = "../../../src/pb/subtivity.v1.rs"]
#[allow(dead_code)]
pub mod pb;
pub use self::pb::BlockStats;

use std::collections::HashSet;

use substreams::Hex;
use substreams::errors::Error;

#[substreams::handlers::map]
pub fn map_block_stats(block: Block) -> Result<BlockStats, Error> {
    let mut transaction_count: i64 = 0;
    let mut event_count: i64 = 0;
    let mut unique_froms = HashSet::new();

    for transaction in block.transactions {
        transaction_count += 1;
        event_count += transaction.events.len() as i64;
        for events in &transaction.events {
            unique_froms.insert(Hex(&events.from_addr).to_string());
        }
    }

    Ok(BlockStats {
        transaction_traces: transaction_count,
        trace_calls: event_count,
        uaw: unique_froms.into_iter().collect(),
    })
}
