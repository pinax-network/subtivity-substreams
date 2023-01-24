#![allow(dead_code, unused)]

use substreams_antelope_core::pb::antelope::{Block};

pub static SECOND: i64 = 1;
pub static MINUTE: i64 = 60;
pub static HOUR: i64 = 3600;
pub static DAY: i64 = 86400;
pub static WEEK: i64 = 604800;

pub fn to_seconds(block: Block) -> i64 {
    block.header.as_ref().unwrap().timestamp.clone().unwrap().seconds
}

pub fn to_nanos(block: Block) -> i32 {
    block.header.as_ref().unwrap().timestamp.clone().unwrap().nanos
}

// pub fn get_database_key(chain: String, interval: i64, seconds: i64) -> String {
//     format!("stats:{}:{}:{}", chain, interval, seconds ) // stats:EOS:86400:1673654400
// }

pub fn get_second_key(seconds: i64) -> String {
    get_key("second", seconds, SECOND)
}

pub fn get_minute_key(seconds: i64) -> String {
    get_key("minute", seconds, MINUTE)
}

pub fn get_hour_key(seconds: i64) -> String {
    get_key("hour", seconds, HOUR)
}

pub fn get_day_key(seconds: i64) -> String {
    get_key("day", seconds, DAY)
}

pub fn get_week_key(seconds: i64) -> String {
    get_key("week", seconds, WEEK)
}

pub fn get_all_key() -> String {
    "all".to_string()
}

pub fn get_rem_euclid(seconds: i64, interval: i64) -> i64 {
    return if seconds % interval == 0 {
        seconds.clone()
    } else {
        seconds.clone() - seconds.rem_euclid(interval)
    };
}

pub fn get_key(suffix: &str, seconds: i64, interval: i64) -> String {
    format!("{}:{}", suffix, get_rem_euclid(seconds, interval).to_string())
}

#[test]
fn seconds_0() {
    let seconds = 0;
    assert_eq!("minute:0", get_minute_key(seconds));
    assert_eq!("hour:0", get_hour_key(seconds));
    assert_eq!("day:0", get_day_key(seconds));
    assert_eq!("week:0", get_week_key(seconds));
}

#[test]
fn seconds_7200() {
    let seconds = 7200;
    assert_eq!("minute:7200", get_minute_key(seconds));
    assert_eq!("hour:7200", get_hour_key(seconds));
    assert_eq!("day:0", get_day_key(seconds));
    assert_eq!("week:0", get_week_key(seconds));
}

#[test]
fn seconds_1209600() {
    let seconds = 1209600;
    assert_eq!("minute:1209600", get_minute_key(seconds));
    assert_eq!("hour:1209600", get_hour_key(seconds));
    assert_eq!("day:1209600", get_day_key(seconds));
    assert_eq!("week:1209600", get_week_key(seconds));
}

#[test]
fn seconds_1673700000() {
    let seconds = 1673700000;
    assert_eq!("minute:1673700000", get_minute_key(seconds));
    assert_eq!("hour:1673697600", get_hour_key(seconds));
    assert_eq!("day:1673654400", get_day_key(seconds));
    assert_eq!("week:1673481600", get_week_key(seconds));
}

// #[test]
// fn key_14400() {
//     assert_eq!(14400, get_key(14400));
// }

