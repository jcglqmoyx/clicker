use enigo::Button;
use fltk::menu::Choice;
use fltk::prelude::{MenuExt, WidgetExt};

use crate::state::{Mode, STATE};

pub(crate) fn register_button_to_click_choice(choice: &mut Choice) {
    choice.set_callback(|c| {
        let button = match c.value() {
            0 => Button::Left,
            1 => Button::Middle,
            2 => Button::Right,
            _ => Button::Left,
        };
        let mut state = STATE.lock().unwrap();
        state.button_to_click = button;
        if state.clicking_mode == Mode::IntelligentMode {
            if state.events.is_empty() {
                return;
            }
            let last = state.events.len() - 1;
            state.events[last].button = button;
        }
    });
}

pub(crate) fn register_mouse_click_type_choice(choice: &mut Choice) {
    choice.set_callback(|c| {
        let click_type = match c.value() {
            0 => 1,
            1 => 2,
            _ => 1,
        };
        let mut state = STATE.lock().unwrap();
        state.mouse_click_type = click_type;
        if state.clicking_mode == Mode::IntelligentMode {
            if state.events.is_empty() {
                return;
            }
            let last = state.events.len() - 1;
            state.events[last].click_type = click_type;
        }
    });
}
