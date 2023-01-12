use substreams::{prelude::*, log};
use substreams::errors::Error;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use substreams_antelope_core::pb::antelope::{Block};

mod pb;
use pb::subtivity_common::{BlockStats, Stats, HourlyStats}; 

/// Extracts the stats from each block
#[substreams::handlers::map]
fn map_block_stats(block: Block) -> Result<BlockStats, Error> {
    Ok(BlockStats {
        block_num: block.number,
        chain: "EOS".parse().unwrap(),
        timestamp: Some(block.header.as_ref().unwrap().timestamp.clone().unwrap()),
        trx_count: block.transaction_traces_count() as i64,
        act_count: block.executed_total_action_count() as i64,
    })
}

// #[substreams::handlers::store]
// fn store_trx_count(block: Block, s: StoreSet ) {
//     log::debug!("chain initialized {}", block_stats.block_num, block_stats.trx_count);
//     s.add(1, get_key(block_stats.block_num.clone()).to_string(), block_stats.trx_count.clone() as i64)
// }

#[substreams::handlers::store]
fn store_trx_count(block_stats: BlockStats, s: StoreAddInt64) {
    log::debug!("block {}: adding transaction count {}", block_stats.block_num, block_stats.trx_count);
    s.add(1, get_key(block_stats.block_num.clone()).to_string(), block_stats.trx_count.clone() as i64)
}

#[substreams::handlers::store]
fn store_act_count(block_stats: BlockStats, s: StoreAddInt64) {
    log::debug!("block {}: adding action count {}", block_stats.block_num, block_stats.act_count);
    s.add(1, get_key(block_stats.block_num.clone()).to_string(), block_stats.act_count as i64)
}

#[substreams::handlers::store]
fn store_max_trx_count(block_stats: BlockStats, s: StoreMaxInt64) {
    log::debug!("block {}: storing max trx count {}", block_stats.block_num, block_stats.trx_count);
    s.max(1, "get_key(block_stats.block_num.clone()).to_string()", block_stats.trx_count as i64);
}

#[substreams::handlers::store]
fn store_max_action_count(block_stats: BlockStats, s: StoreMaxInt64) {
    log::debug!("block {}: storing max action count {}", block_stats.block_num, block_stats.act_count);
    s.max(1, get_key(block_stats.block_num.clone()).to_string(), block_stats.act_count as i64);
}

/// Emits hourly accumulated stats
#[substreams::handlers::map]
fn map_hourly_stats(
    block_stats: BlockStats,
    store_trx_count: StoreGetInt64,
    store_act_count: StoreGetInt64,
) -> Result<Stats, Error> {
    let mut res: Vec<HourlyStats> = Vec::new();

    // this is the last block of the block range, emit the accumulated stats
    // todo replace this with hourly buckets based on timestamp as we can't be sure there is exactly 7200 blocks within one hour
    if (block_stats.block_num + 1) % 7200 == 0 {
        // log::debug!("block stats of block_num {} for key {} with trx_count {} and act_count {}", block_stats.block_num, get_key(block_stats.block_num), store_trx_count.get_at(1, get_key(block_stats.block_num).to_string()).unwrap(), store_act_count.get_at(1, get_key(block_stats.block_num).to_string()).unwrap());
        res.push(HourlyStats {
            block_num: get_key(block_stats.block_num),
            chain: block_stats.chain,
            timestamp: block_stats.timestamp,
            trx_count: store_trx_count.get_at(1, get_key(block_stats.block_num).to_string()).unwrap(),
            act_count: store_act_count.get_at(1, get_key(block_stats.block_num).to_string()).unwrap(),
        })
    }

    Ok(Stats { stats: res })
}

#[substreams::handlers::map]
pub fn db_out(
    block_stats: BlockStats,
    stats: Stats,
    // store_max_trx_count: Deltas<DeltaInt64>,
    // store_max_action_count: Deltas<DeltaInt64>,
) -> Result<DatabaseChanges, Error> {
    let mut database_changes: DatabaseChanges = Default::default();

    // if we have hourly accumulated stats, push them to the hourly_stats table
    for stat in stats.stats {
        database_changes.push_change("hourly_stats", &*(stat.block_num).to_string(), 0, Operation::Create)
            .change("chain", (stat.chain.clone(), stat.chain.clone()))
            .change("trx_count", (0, stat.trx_count))
            .change("act_count", (0, stat.act_count));
    }

    // update the last_block table with the latest block
    let mut last_block_operation = Operation::Update;

    if block_stats.block_num == 2 { // if this is the first block we need to create the entry first
        last_block_operation = Operation::Create;
    }
    database_changes.push_change("last_block", &*block_stats.chain, 0, last_block_operation)
        .change("block_num", (block_stats.block_num.clone(), block_stats.block_num.clone()))
        .change("trx_count", (0, block_stats.trx_count))
        .change("act_count", (0, block_stats.act_count));

    // for delta in store_max_trx_count.deltas {
    //     // update the max_trx_block table in case we found a new trx maximum
    //     if delta.operation == substreams::pb::substreams::store_delta::Operation::Create {
    //         database_changes.push_change("max_trx_block", &*delta.key, 0, Operation::Create)
    //             .change("chain", (blk_stats.chain.clone(), blk_stats.chain.clone()))
    //             .change("trx_count", (0, delta.new_value));
    //     } else if delta.operation == substreams::pb::substreams::store_delta::Operation::Update {
    //         database_changes.push_change("max_trx_block", &*delta.key, 0, Operation::Update)
    //             .change("chain", (blk_stats.chain.clone(), blk_stats.chain.clone()))
    //             .change("trx_count", (delta.old_value, delta.new_value));
    //     }
    // }
    //
    // for delta in store_max_action_count.deltas {
    //     // update the max_action_block table in case we found a new action maximum
    //     if delta.operation == substreams::pb::substreams::store_delta::Operation::Create {
    //         database_changes.push_change("max_action_block", &*delta.key, 0, Operation::Create)
    //             .change("chain", (blk_stats.chain.clone(), blk_stats.chain.clone()))
    //             .change("act_count", (0, delta.new_value));
    //     } else if delta.operation == substreams::pb::substreams::store_delta::Operation::Update {
    //         database_changes.push_change("max_action_block", &*delta.key, 0, Operation::Update)
    //             .change("chain", (blk_stats.chain.clone(), blk_stats.chain.clone()))
    //             .change("act_count", (delta.old_value, delta.new_value));
    //     }
    // }

    Ok(database_changes)
}

fn get_key(block_num: u32) -> u32 {
    return if block_num % 7200 == 0 {
        block_num.clone()
    } else {
        block_num.clone() - block_num.rem_euclid(7200)
    };
}

#[test]
fn block_num_key_0() {
    assert_eq!(0, get_key(0));
}

#[test]
fn block_num_key_2() {
    assert_eq!(0, get_key(2));
}

#[test]
fn block_num_key_7200() {
    assert_eq!(7200, get_key(7200));
}

#[test]
fn block_num_key_8000() {
    assert_eq!(7200, get_key(8000));
}

#[test]
fn block_num_key_14400() {
    assert_eq!(14400, get_key(14400));
}

