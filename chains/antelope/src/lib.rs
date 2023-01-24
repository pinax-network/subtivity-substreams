use substreams::{prelude::*, log};
use substreams::errors::Error;
use substreams_antelope_core::pb::antelope::{Block};
use subtivity_common::subtivity::{Counter, Counters};

// use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
// use substreams_sink_kv::pb::kv::KvOperations;

mod keyer;

#[substreams::handlers::store]
fn store_traces_count(block: Block, s: StoreAddInt64) {
    let traces_count = block.transaction_traces_count() as i64;
    let seconds = keyer::to_seconds(block.clone());
    log::debug!("block {}: seconds {}: adding transaction traces count {}", block.number, seconds, traces_count);

    // s.add(1, keyer::get_second_key(seconds), traces_count);
    // s.add(1, keyer::get_minute_key(seconds), traces_count);
    s.add(1, keyer::get_hour_key(seconds), traces_count);
    s.add(1, keyer::get_day_key(seconds), traces_count);
    s.add(1, keyer::get_week_key(seconds), traces_count);
    s.add(1, keyer::get_all_key(), traces_count)
}

#[substreams::handlers::store]
fn store_action_count(block: Block, s: StoreAddInt64) {
    let action_count = block.executed_total_action_count() as i64;
    let seconds = keyer::to_seconds(block.clone());
    log::debug!("block {}: seconds {}: adding executed total action count {}", block.number, seconds, action_count);

    // s.add(1, keyer::get_second_key(seconds), action_count);
    // s.add(1, keyer::get_minute_key(seconds), action_count);
    s.add(1, keyer::get_hour_key(seconds), action_count);
    s.add(1, keyer::get_day_key(seconds), action_count);
    s.add(1, keyer::get_week_key(seconds), action_count);
    s.add(1, keyer::get_all_key(), action_count)
}

#[substreams::handlers::map]
pub fn map_hour_counters(
    block: Block,
    store_traces_count: StoreGetInt64,
    store_action_count: StoreGetInt64
) -> Result<Counters, Error> {
    computer_counters(block, store_traces_count, store_action_count, "hour", keyer::HOUR)
}

#[substreams::handlers::map]
pub fn map_day_counters(
    block: Block,
    store_traces_count: StoreGetInt64,
    store_action_count: StoreGetInt64
) -> Result<Counters, Error> {
    computer_counters(block, store_traces_count, store_action_count, "day", keyer::DAY)
}

#[substreams::handlers::map]
pub fn map_week_counters(
    block: Block,
    store_traces_count: StoreGetInt64,
    store_action_count: StoreGetInt64
) -> Result<Counters, Error> {
    computer_counters(block, store_traces_count, store_action_count, "week", keyer::WEEK)
}

fn computer_counters(
    block: Block,
    store_traces_count: StoreGetInt64,
    store_action_count: StoreGetInt64,
    suffix: &str,
    interval: i64
) -> Result<Counters, Error> {
    // global counters
    let mut counters = vec![];
    let mut seconds = keyer::to_seconds(block.clone()) - interval; // start at -1 interval
    let mut max_counters = 730; // 30 days / 2 years / 14 years

    // get global counters
    let traces_count = store_traces_count.get_at(1, keyer::get_all_key());
    let action_count = store_action_count.get_at(1, keyer::get_all_key());

    // break if global counters are not available
    if traces_count.is_none() || action_count.is_none() { return Ok(Counters::default()) }

    // generate daily counters
    while max_counters > 0 {
        // get counters
        let key = keyer::get_key(suffix, seconds, interval).to_string();
        let traces_count = store_traces_count.get_at(1, &key);
        let action_count = store_action_count.get_at(1, &key);

        // break if counters are not available
        if traces_count.is_none() || action_count.is_none() { break; }
        
        // add counter
        counters.push(Counter {
            seconds,
            action_count: action_count.unwrap(),
            traces_count: traces_count.unwrap(),
        });

        // next counter
        seconds -= interval;
        max_counters -= 1;
    }

    Ok(Counters{
        total_action_count: action_count.unwrap(),
        total_traces_count: traces_count.unwrap(),
        counters
    })
}

// #[substreams::handlers::map]
// pub fn db_out(
//     block: Block,
//     store_traces_count: StoreGetInt64,
//     store_action_count: StoreGetInt64
// ) -> Result<DatabaseChanges, Error> {
//     let mut database_changes: DatabaseChanges = Default::default();
//     if keyer::to_nanos(block.clone()) != 0 { return Ok(database_changes); } // Antelope Chains have 0.5s block time (skip even blocks)

//     let seconds = keyer::to_seconds(block.clone());
//     let chain = "EOS".to_string();

//     // stats for each block
//     database_changes.push_change("stats", &keyer::get_database_key(chain.clone(), keyer::SECOND, seconds), 0, Operation::Create)
//         .change("chain", (chain.clone(), chain.clone()))
//         .change("block_num", (0, block.number))
//         .change("seconds", (0, seconds))
//         .change("interval", (0, 1))
//         .change("traces_count", (0, store_traces_count.get_at(1, keyer::get_second_key(seconds)) ))
//         .change("action_count", (0, store_action_count.get_at(1, keyer::get_second_key(seconds)) ));
    
//     Ok(database_changes)
// }

// #[substreams::handlers::map]
// pub fn kv_out( block: Block ) -> Result<KvOperations, Error> {

//     let mut kv_ops: KvOperations = Default::default();

//     kv_ops.push_new(block.number.to_string(), block.id, 1);

//     Ok(kv_ops)
// }
