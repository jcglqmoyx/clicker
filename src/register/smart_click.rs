use fltk::button::CheckButton;
use fltk::prelude::WidgetExt;

use crate::global::mode::{CLICKING_MODE, Mode};

pub(crate) fn toggle_click_mode(button: &mut CheckButton) {
    button.set_callback(|_| unsafe {
        if CLICKING_MODE == Mode::OrdinaryMode {
            CLICKING_MODE = Mode::IntelligentMode;
        } else if CLICKING_MODE == Mode::IntelligentMode {
            CLICKING_MODE = Mode::OrdinaryMode;
        }
    });
}

pub(crate) fn toggle_play_record_sound(button: &mut CheckButton) {
    button.set_callback(|_| unsafe {
        if crate::global::mode::PLAY_RECORD_SOUND {
            crate::global::mode::PLAY_RECORD_SOUND = false;
        } else {
            crate::global::mode::PLAY_RECORD_SOUND = true;
        }
    });
}