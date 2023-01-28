#![allow(dead_code, unused)]

pub static ZERO: i64 = 0;
pub static SECOND: i64 = 1;
pub static MINUTE: i64 = 60;
pub static HOUR: i64 = 3600;
pub static DAY: i64 = 86400;
pub static WEEK: i64 = 604800;
pub static INTERVALS: [i64; 6] = [ZERO, SECOND, MINUTE, HOUR, DAY, WEEK];

pub fn get_all_key() -> String {
    get_key(0, 0)
}

pub fn get_second_key(seconds: i64) -> String {
    get_key(seconds, SECOND)
}

pub fn get_minute_key(seconds: i64) -> String {
    get_key(seconds, MINUTE)
}

pub fn get_hour_key(seconds: i64) -> String {
    get_key(seconds, HOUR)
}

pub fn get_day_key(seconds: i64) -> String {
    get_key(seconds, DAY)
}

pub fn get_week_key(seconds: i64) -> String {
    get_key(seconds, WEEK)
}

pub fn get_rem_euclid(seconds: i64, interval: i64) -> i64 {
    if interval == 0 { return 0; }
    return if seconds % interval == 0 {
        seconds.clone()
    } else {
        seconds.clone() - seconds.rem_euclid(interval)
    };
}

pub fn get_key(seconds: i64, interval: i64) -> String {
    format!("{}:{}", interval, get_rem_euclid(seconds, interval).to_string())
}

#[test]
fn seconds_0() {
    let seconds = 0;
    assert_eq!("0:0", get_all_key());
    assert_eq!("60:0", get_minute_key(seconds));
    assert_eq!("3600:0", get_hour_key(seconds));
    assert_eq!("86400:0", get_day_key(seconds));
    assert_eq!("604800:0", get_week_key(seconds));
}

#[test]
fn seconds_7200() {
    let seconds = 7200;
    assert_eq!("60:7200", get_minute_key(seconds));
    assert_eq!("3600:7200", get_hour_key(seconds));
    assert_eq!("86400:0", get_day_key(seconds));
    assert_eq!("604800:0", get_week_key(seconds));
}

#[test]
fn seconds_7201() {
    let seconds = 7201;
    assert_eq!("60:7200", get_minute_key(seconds));
    assert_eq!("3600:7200", get_hour_key(seconds));
    assert_eq!("86400:0", get_day_key(seconds));
    assert_eq!("604800:0", get_week_key(seconds));
}

#[test]
fn seconds_1209600() {
    let seconds = 1209600;
    assert_eq!("60:1209600", get_minute_key(seconds));
    assert_eq!("3600:1209600", get_hour_key(seconds));
    assert_eq!("86400:1209600", get_day_key(seconds));
    assert_eq!("604800:1209600", get_week_key(seconds));
}

#[test]
fn seconds_1673700000() {
    let seconds = 1673700000;
    assert_eq!("60:1673700000", get_minute_key(seconds));
    assert_eq!("3600:1673697600", get_hour_key(seconds));
    assert_eq!("86400:1673654400", get_day_key(seconds));
    assert_eq!("604800:1673481600", get_week_key(seconds));
}
