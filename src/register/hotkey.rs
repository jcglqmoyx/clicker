use fltk::{app, enums};
use fltk::input::Input;
use fltk::prelude::{InputExt, WidgetBase, WidgetExt};
use crate::global::hotkey::{CLICK_HOT_KEY, Key, LAST_TIME_CLICK_HOT_KEY_CHANGED, LAST_TIME_RECORD_HOT_KEY_CHANGED, RECORD_HOT_KEY};
use crate::utils::key;
use crate::utils::key::get_hotkey_string;
use crate::utils::time::timestamp;

pub(crate) fn register_hot_key(input_widget: &mut Input, hot_key_type:i32) {
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
        enums::Event::KeyDown => unsafe {
            let key_int_value = app::event_key().bits();
            let key_name = key::get_key_name_in_fltk(key_int_value);
            let now = timestamp();

            if hot_key_type == 1 {
                if now - LAST_TIME_CLICK_HOT_KEY_CHANGED < 100 && CLICK_HOT_KEY.len() < 4 {
                    CLICK_HOT_KEY.insert(Key::new(key_int_value, key_name));
                } else {
                    CLICK_HOT_KEY.clear();
                    CLICK_HOT_KEY.insert(Key::new(key_int_value, key_name));
                }
                if *CLICK_HOT_KEY == *RECORD_HOT_KEY {
                    CLICK_HOT_KEY.clear();
                    input.set_value("");
                } else {
                    input.set_value(&get_hotkey_string(&CLICK_HOT_KEY));
                }
                LAST_TIME_CLICK_HOT_KEY_CHANGED = now;
                true
            } else if hot_key_type == 2 {
                if now - LAST_TIME_RECORD_HOT_KEY_CHANGED < 100 && RECORD_HOT_KEY.len() < 4 {
                    RECORD_HOT_KEY.insert(Key::new(key_int_value, key_name));
                } else {
                    RECORD_HOT_KEY.clear();
                    RECORD_HOT_KEY.insert(Key::new(key_int_value, key_name));
                }
                if *RECORD_HOT_KEY == *CLICK_HOT_KEY {
                    RECORD_HOT_KEY.clear();
                    input.set_value("");
                } else {
                    input.set_value(&get_hotkey_string(&RECORD_HOT_KEY));
                }
                LAST_TIME_RECORD_HOT_KEY_CHANGED = now;
                true
            } else {
                false
            }
        }
        _ => false,
    });
}