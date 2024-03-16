use crate::persistence::entity::event::Event;

#[derive(Debug, Clone)]
pub(crate) struct Record {
    pub(crate) id: Option<i32>,
    pub(crate) title: String,
    pub(crate) events: Vec<Event>,
}

impl Record {
    pub(crate) fn new(title: String, events: Vec<Event>) -> Self {
        Self {
            id: None,
            title,
            events,
        }
    }
}