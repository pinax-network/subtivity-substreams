#[path = "../../../src/pb/subtivity.v1.rs"]
#[allow(dead_code)]
pub mod pb;
pub use pb::*;
use std::collections::HashSet;

use substreams::errors::Error;
use substreams_near::pb::sf::near::r#type::v1::Block;

#[substreams::handlers::map]
pub fn map_block_stats(block: Block) -> Result<BlockStats, Error> {
    let mut transaction_count: i64 = 0;
    let mut action_count: i64 = 0;
    let mut unique_signer_ids = HashSet::new();

    for shard in block.shards {
        if let Some(chunk) = shard.chunk {
            transaction_count += chunk.transactions.len() as i64;

            for transaction in chunk.transactions {
                if let Some(trans) = transaction.transaction {
                    action_count += trans.actions.len() as i64;
                    unique_signer_ids.insert(trans.signer_id);
                }
            }
        }
    }

    Ok(BlockStats {
        transaction_traces: transaction_count,
        trace_calls: action_count,
        uaw: unique_signer_ids.into_iter().collect(),
    })
}
