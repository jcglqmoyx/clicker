use fltk::button::CheckButton;
use fltk::prelude::{ButtonExt, WidgetExt};

use crate::state::{Mode, STATE};

pub(crate) fn toggle_click_mode(button: &mut CheckButton) {
    button.set_callback(|b| {
        let mut state = STATE.lock().unwrap();
        state.clicking_mode = if b.value() {
            Mode::IntelligentMode
        } else {
            Mode::OrdinaryMode
        };
    });
}

pub(crate) fn toggle_enable_sound_effect(button: &mut CheckButton) {
    button.set_callback(|b| {
        let mut state = STATE.lock().unwrap();
        state.enable_sound = b.value();
    });
}
