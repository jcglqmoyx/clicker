use fltk::input::Input;
use fltk::prelude::WidgetBase;
use once_cell::sync::Lazy;

use crate::persistence::entity::event::Event;

pub(crate) static mut EVENTS: Vec<Event> = vec![];

pub(crate) static mut COUNT_RECORD_INPUT: Lazy<Input> = Lazy::new(|| Input::new(50, 300, 60, 0, ""));