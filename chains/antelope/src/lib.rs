use substreams::{prelude::*, log};
use substreams::errors::Error;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use substreams_antelope_core::pb::antelope::{Block};

mod pb;
// use pb::subtivity_common::{Stat, Stats}; 

#[substreams::handlers::store]
fn store_traces_count(block: Block, s: StoreAddInt64) {
    let key = get_key(to_timestamp(block.clone())).to_string();
    let traces_count = block.transaction_traces_count() as i64;
    log::debug!("key {}: adding transaction traces count {}", key, traces_count);
    s.add(1, key, traces_count)
}

#[substreams::handlers::store]
fn store_action_count(block: Block, s: StoreAddInt64) {
    let key = get_key(to_timestamp(block.clone())).to_string();
    let action_count = block.executed_total_action_count() as i64;
    log::debug!("key {}: adding executed total action count {}", key, action_count);
    s.add(1, key, action_count)
}

// // Emits hourly accumulated stats
// #[substreams::handlers::map]
// fn map_hourly_stats(
//     block: Block,
//     store_transaction_traces_count: StoreGetInt64,
//     store_executed_total_action_count: StoreGetInt64
// ) -> Result<Stats, Error> {
//     let mut res: Vec<HourlyStats> = Vec::new();

//     // this is the last block of the block range, emit the accumulated stats
//     // todo replace this with hourly buckets based on timestamp as we can't be sure there is exactly 7200 blocks within one hour
//     if (stat.block_num + 1) % 7200 == 0 {
//         // log::debug!("block stats of block_num {} for key {} with trx_count {} and act_count {}", stat.block_num, get_key(stat.block_num), store_trx_count.get_at(1, get_key(stat.block_num).to_string()).unwrap(), store_act_count.get_at(1, get_key(stat.block_num).to_string()).unwrap());
//         res.push(HourlyStats {
//             block_num: get_key(stat.block_num),
//             chain: stat.chain,
//             timestamp: stat.timestamp,
//             trx_count: store_trx_count.get_at(1, get_key(stat.block_num).to_string()).unwrap(),
//             act_count: store_act_count.get_at(1, get_key(stat.block_num).to_string()).unwrap(),
//         })
//     }

//     Ok(Stats { stats: res })
// }

#[substreams::handlers::map]
pub fn db_out(
    block: Block,
    store_traces_count: StoreGetInt64,
    store_action_count: StoreGetInt64
) -> Result<DatabaseChanges, Error> {
    let mut database_changes: DatabaseChanges = Default::default();
    
    let timestamp = to_timestamp(block.clone());
    let key = get_key(timestamp);
    if block.number % 2 == 0 { return Ok(database_changes); } // Antelope Chains have 0.5s block time (skip even blocks)
    if timestamp != key { return Ok(database_changes); }
    log::debug!("key {}: hourly database output", key);

    // get store
    let traces_count = store_traces_count.get_at(1, key.to_string()).unwrap();
    let action_count = store_action_count.get_at(1, key.to_string()).unwrap();

    // if we have hourly accumulated stats, push them to the [hourly_stats] table
    database_changes.push_change("hourly_stats", &key.to_string(), 0, Operation::Create)
        .change("chain", ("EOS", "EOS"))
        .change("block_num", (0, block.number))
        .change("traces_count", (0, traces_count))
        .change("action_count", (0, action_count));

    Ok(database_changes)
}

fn to_timestamp(block: Block) -> i64 {
    block.header.as_ref().unwrap().timestamp.clone().unwrap().seconds
}

fn get_key(timestamp: i64) -> i64 {
    let interval = 3600;
    return if timestamp % interval == 0 {
        timestamp.clone()
    } else {
        timestamp.clone() - timestamp.rem_euclid(interval)
    };
}

// #[substreams::handlers::store]
// fn store_executed_total_action_count(stat: Stat, s: StoreAddInt64) {
//     log::debug!("block {}: adding action count {}", stat.block_num, stat.executed_total_action_count);
//     s.add(1, get_key(stat.timestamp).to_string(), stat.executed_total_action_count as i64)
// }

// #[substreams::handlers::store]
// fn store_max_trx_count(stat: Stat, s: StoreMaxInt64) {
//     log::debug!("block {}: storing max trx count {}", stat.block_num, stat.trx_count);
//     s.max(1, "get_key(stat.timestamp).to_string()", stat.trx_count as i64);
// }

// #[substreams::handlers::store]
// fn store_max_action_count(stat: Stat, s: StoreMaxInt64) {
//     log::debug!("block {}: storing max action count {}", stat.block_num, stat.act_count);
//     s.max(1, get_key(stat.timestamp).to_string(), stat.act_count as i64);
// }

// #[substreams::handlers::map]
// pub fn db_out( stat: Stat, stats: Stats ) -> Result<DatabaseChanges, Error> {
//     let mut database_changes: DatabaseChanges = Default::default();

//     // if we have hourly accumulated stats, push them to the hourly_stats table
//     for stat in stats.stats {
//         database_changes.push_change("hourly_stats", &*(stat.block_num).to_string(), 0, Operation::Create)
//             .change("chain", (stat.chain.clone(), stat.chain.clone()))
//             .change("trx_count", (0, stat.trx_count))
//             .change("act_count", (0, stat.act_count));
//     }

//     // update the last_block table with the latest block
//     let mut last_block_operation = Operation::Update;

//     if stat.block_num == 2 { // if this is the first block we need to create the entry first
//         last_block_operation = Operation::Create;
//     }
//     database_changes.push_change("last_block", &*stat.chain, 0, last_block_operation)
//         .change("block_num", (stat.timestamp, stat.timestamp))
//         .change("trx_count", (0, stat.trx_count))
//         .change("act_count", (0, stat.act_count));

//     // for delta in store_max_trx_count.deltas {
//     //     // update the max_trx_block table in case we found a new trx maximum
//     //     if delta.operation == substreams::pb::substreams::store_delta::Operation::Create {
//     //         database_changes.push_change("max_trx_block", &*delta.key, 0, Operation::Create)
//     //             .change("chain", (blk_stats.chain.clone(), blk_stats.chain.clone()))
//     //             .change("trx_count", (0, delta.new_value));
//     //     } else if delta.operation == substreams::pb::substreams::store_delta::Operation::Update {
//     //         database_changes.push_change("max_trx_block", &*delta.key, 0, Operation::Update)
//     //             .change("chain", (blk_stats.chain.clone(), blk_stats.chain.clone()))
//     //             .change("trx_count", (delta.old_value, delta.new_value));
//     //     }
//     // }
//     //
//     // for delta in store_max_action_count.deltas {
//     //     // update the max_action_block table in case we found a new action maximum
//     //     if delta.operation == substreams::pb::substreams::store_delta::Operation::Create {
//     //         database_changes.push_change("max_action_block", &*delta.key, 0, Operation::Create)
//     //             .change("chain", (blk_stats.chain.clone(), blk_stats.chain.clone()))
//     //             .change("act_count", (0, delta.new_value));
//     //     } else if delta.operation == substreams::pb::substreams::store_delta::Operation::Update {
//     //         database_changes.push_change("max_action_block", &*delta.key, 0, Operation::Update)
//     //             .change("chain", (blk_stats.chain.clone(), blk_stats.chain.clone()))
//     //             .change("act_count", (delta.old_value, delta.new_value));
//     //     }
//     // }

//     Ok(database_changes)
// }

// #[test]
// fn block_num_key_0() {
//     assert_eq!(0, get_key(0));
// }

// #[test]
// fn block_num_key_2() {
//     assert_eq!(0, get_key(2));
// }

// #[test]
// fn block_num_key_7200() {
//     assert_eq!(7200, get_key(7200));
// }

// #[test]
// fn block_num_key_8000() {
//     assert_eq!(7200, get_key(8000));
// }

// #[test]
// fn block_num_key_14400() {
//     assert_eq!(14400, get_key(14400));
// }

