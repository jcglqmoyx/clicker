use crate::persistence::entity::event::Event;

#[derive(Debug)]
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