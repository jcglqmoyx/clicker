use enigo::Button;
use fltk::menu::Choice;
use fltk::prelude::{MenuExt, WidgetExt};

use crate::global::click::EVENTS;
use crate::global::mode::{BUTTON_TO_CLICK, CLICKING_MODE, Mode, MOUSE_CLICK_TYPE};

pub(crate) fn register_button_to_click_choice(choice: &mut Choice) {
    choice.set_callback(|c| unsafe {
        BUTTON_TO_CLICK = match c.value() {
            0 => Button::Left,
            1 => Button::Middle,
            2 => Button::Right,
            _ => { Button::Left }
        };
        if CLICKING_MODE == Mode::IntelligentMode {
            if EVENTS.is_empty() {
                return;
            }
            EVENTS[EVENTS.len() - 1].button = BUTTON_TO_CLICK;
        }
    });
}

pub(crate) fn register_mouse_click_type_choice(choice: &mut Choice) {
    choice.set_callback(|c| unsafe {
        MOUSE_CLICK_TYPE = match c.value() {
            0 => 1,
            1 => 2,
            _ => { 1 }
        };
        if CLICKING_MODE == Mode::IntelligentMode {
            if EVENTS.is_empty() {
                return;
            }
            EVENTS[EVENTS.len() - 1].click_type = MOUSE_CLICK_TYPE;
        }
    });
}