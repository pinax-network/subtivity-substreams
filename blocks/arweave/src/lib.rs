pub mod pb;
use std::collections::HashSet;

use pb::sf::arweave::r#type::v1::Block;
use pb::subtivity::v1::BlockStats;
use substreams::log;
use substreams::Hex;
use substreams::errors::Error;

#[substreams::handlers::map]
fn map_block_stats(blk: Block) -> Result<BlockStats, Error> {
    let mut transactions: i64 = 0;
    let mut unique_owners = HashSet::new();

    for transaction in blk.txs.into_iter() {
        // Count transactions
        transactions += 1;
        // convert owner to hex
        let owner = Hex(&transaction.owner).to_string();
        unique_owners.insert(owner);
    }
    log::debug!("unique_owners={} transactions={}", unique_owners.len(), transactions);

    Ok(BlockStats {
        transaction_traces: transactions as i64,
        trace_calls: transactions as i64,
        uaw: unique_owners.into_iter().collect(),
        blobs: 0,
    })
}
