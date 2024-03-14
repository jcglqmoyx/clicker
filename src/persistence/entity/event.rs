use std::fmt::Debug;

#[derive(Debug)]
pub(crate) struct Event {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) button: enigo::Button,
    pub(crate) click_type: u8,
    pub(crate) sleep: u64,
}