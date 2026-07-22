use fltk::input::Input;
use fltk::prelude::{InputExt, WidgetBase, WidgetExt};
use fltk::{app, enums};

use crate::state::{Key, STATE};
use crate::utils::key;
use crate::utils::key::get_hotkey_string;
use crate::utils::time::timestamp;

pub(crate) fn register_hot_key(input_widget: &mut Input, hot_key_type: i32) {
    input_widget.handle(move |input, event| match event {
        enums::Event::Enter => {
            input.activate();
            app::redraw();
            true
        }
        enums::Event::Leave => {
            input.deactivate();
            app::redraw();
            true
        }
        enums::Event::KeyDown => {
            let key_int_value = app::event_key().bits();
            let key_name = key::get_key_name_in_fltk(key_int_value);
            let now = timestamp();

            let mut state = STATE.lock().unwrap();
            if hot_key_type == 1 {
                if now - state.last_click_hotkey_changed < 100 && state.click_hotkey.len() < 4 {
                    state.click_hotkey.insert(Key::new(key_int_value, key_name));
                } else {
                    state.click_hotkey.clear();
                    state.click_hotkey.insert(Key::new(key_int_value, key_name));
                }
                if state.click_hotkey == state.record_hotkey {
                    state.click_hotkey.clear();
                    input.set_value("");
                } else {
                    input.set_value(&get_hotkey_string(&state.click_hotkey));
                }
                state.last_click_hotkey_changed = now;
                true
            } else if hot_key_type == 2 {
                if now - state.last_record_hotkey_changed < 100 && state.record_hotkey.len() < 4 {
                    state
                        .record_hotkey
                        .insert(Key::new(key_int_value, key_name));
                } else {
                    state.record_hotkey.clear();
                    state
                        .record_hotkey
                        .insert(Key::new(key_int_value, key_name));
                }
                if state.record_hotkey == state.click_hotkey {
                    state.record_hotkey.clear();
                    input.set_value("");
                } else {
                    input.set_value(&get_hotkey_string(&state.record_hotkey));
                }
                state.last_record_hotkey_changed = now;
                true
            } else {
                false
            }
        }
        _ => false,
    });
}
