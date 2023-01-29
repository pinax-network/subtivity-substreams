pub static INTERVAL: i64 = 86400;

pub fn get_rem_euclid(seconds: i64, interval: i64) -> i64 {
    if interval == 0 {
        return 0;
    }
    if seconds % interval == 0 {
        return seconds;
    }
    seconds - seconds.rem_euclid(interval)
}

pub fn get_key(seconds: i64, interval: i64) -> String {
    format!("{}:{}", interval, get_rem_euclid(seconds, interval))
}

#[test]
fn test_get_key() {
    assert_eq!("86400:0", get_key(0, 86400));
    assert_eq!("86400:86400", get_key(86400, 86400));
    assert_eq!("86400:172800", get_key(172800, 86400));
    assert_eq!("86400:1528502400", get_key(1528502400, 86400));
}
