use std::collections::HashSet;

use once_cell::sync::Lazy;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub(crate) struct Key {
    pub(crate) int_value: i32,
    pub(crate) string_value: String,
}

impl Key {
    pub(crate) fn new(int_value: i32, string_value: String) -> Self {
        Key {
            int_value,
            string_value,
        }
    }
}

pub(crate) static mut CLICK_HOT_KEY: Lazy<HashSet<Key>> = Lazy::new(HashSet::new);

pub(crate) static mut RECORD_HOT_KEY: Lazy<HashSet<Key>> = Lazy::new(HashSet::new);

pub(crate) static mut LAST_TIME_CLICK_HOT_KEY_CHANGED: u64 = 0;

pub(crate) static mut LAST_TIME_RECORD_HOT_KEY_CHANGED: u64 = 0;