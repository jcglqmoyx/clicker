use std::fmt::Debug;

#[derive(Debug)]
pub(crate) struct Event {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) button: enigo::Button,
    pub(crate) click_type: u8,
    pub(crate) sleep: u64,
}

pub(crate) struct Record {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) events: Vec<Event>,
}

impl Record {
    pub(crate) fn new(id: i32, title: String, description: String, events: Vec<Event>) -> Self {
        Self {
            id,
            title,
            description,
            events,
        }
    }
}