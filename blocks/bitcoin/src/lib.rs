#[path = "../../../src/pb/subtivity.v1.rs"]
#[allow(dead_code)]
pub mod pb;
pub use self::pb::BlockStats;

use std::collections::HashSet;

use substreams::log;
use substreams::errors::Error;
use substreams_bitcoin::pb::btc::v1::Block;

#[substreams::handlers::map]
pub fn map_block_stats(block: Block) -> Result<BlockStats, Error> {
    let mut transactions: i64 = 0;
    let mut outputs: i64 = 0;
    let mut unique_addresses = HashSet::new();

    for transaction in block.transactions() {
        // Count transactions
        transactions += 1;

        // Count outputs
        outputs += transaction.vout.len() as i64;

        // Add unique addresses
        for vout in transaction.vout.clone().into_iter() {
            match vout.script_pub_key {
                Some(script_pub_key) => {
                    if !script_pub_key.address.is_empty() {
                        unique_addresses.insert(script_pub_key.address);
                    }
                    script_pub_key.addresses.into_iter().for_each(|address| {
                        unique_addresses.insert(address);
                    });
                }
                None => {}
            }
        }
    }
    log::debug!("unique_addresses={} outputs={} transactions={}", unique_addresses.len(), outputs, transactions);

    Ok(BlockStats {
        transaction_traces: transactions as i64,
        trace_calls: outputs as i64,
        uaw: unique_addresses.into_iter().collect(),
        blobs: 0,
    })
}
