use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn timestamp() -> u64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    now.as_secs() * 1000 + now.subsec_millis() as u64
}

pub(crate) fn since(last_time: u64) -> u64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let now = now.as_secs() * 1000 + now.subsec_millis() as u64;
    now - last_time
}