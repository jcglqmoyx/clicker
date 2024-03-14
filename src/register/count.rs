use fltk::button::CheckButton;
use fltk::enums;
use fltk::input::IntInput;
use fltk::prelude::{InputExt, WidgetBase, WidgetExt};

use crate::global::count::{CLICK_COUNT, COUNTER_ENABLED};

pub(crate) fn toggle_click_counter(button: &mut CheckButton) {
    button.set_callback(|_| unsafe {
        COUNTER_ENABLED = !COUNTER_ENABLED;
    });
}

pub(crate) fn change_click_count(input: &mut IntInput) {
    input.handle(move |input, event|
        match event {
            enums::Event::KeyDown => unsafe {
                if input.value().is_empty() {
                    CLICK_COUNT = 0;
                } else if input.value().chars().next() == Option::from('-') {
                    CLICK_COUNT = 0;
                    input.set_value("0");
                } else if input.value().len() >= 10 {
                    input.set_value(&i32::MAX.to_string());
                    CLICK_COUNT = i32::MAX;
                } else {
                    CLICK_COUNT = input.value().parse().unwrap();
                }
                true
            }
            _ => {
                false
            }
        });
}