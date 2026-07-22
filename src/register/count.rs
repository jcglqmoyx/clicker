use fltk::button::CheckButton;
use fltk::enums;
use fltk::input::IntInput;
use fltk::prelude::{ButtonExt, InputExt, WidgetBase, WidgetExt};

use crate::state::STATE;

pub(crate) fn toggle_click_counter(button: &mut CheckButton) {
    button.set_callback(|b| {
        let mut state = STATE.lock().unwrap();
        state.counter_enabled = b.value();
    });
}

pub(crate) fn change_click_count(input: &mut IntInput) {
    input.handle(move |input, event| match event {
        enums::Event::KeyDown => {
            let mut state = STATE.lock().unwrap();
            if input.value().is_empty() {
                state.click_count = 0;
            } else if input.value().chars().next() == Option::from('-') {
                state.click_count = 0;
                input.set_value("0");
            } else if input.value().len() >= 10 {
                input.set_value(&i32::MAX.to_string());
                state.click_count = i32::MAX;
            } else {
                state.click_count = input.value().parse().unwrap();
            }
            true
        }
        _ => false,
    });
}
