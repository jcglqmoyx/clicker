use crate::persistence::entity::event::Event;

pub(crate) static mut EVENTS: Vec<Event> = vec![];
