use fltk::enums::CallbackTrigger;
use fltk::frame::Frame;
use fltk::group::{Pack, PackType};
use fltk::input::Input;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};

use crate::register::hotkey::register_hot_key;

pub fn hotkey_settings_panel() -> Pack {
    let mut panel = Pack::new(300, 240, 200, 100, "Hotkey");
    panel.set_spacing(10);
    panel.set_type(PackType::Vertical);

    let mut click_hotkey_panel = Pack::new(0, 0, 200, 30, "");
    click_hotkey_panel.set_type(PackType::Horizontal);

    let click_label = Frame::new(0, 0, 50, 30, "Click");
    click_hotkey_panel.add(&click_label);

    let mut click_hotkey_input = Input::new(50, 0, 150, 30, "");
    click_hotkey_input.deactivate();
    click_hotkey_input.set_trigger(CallbackTrigger::Changed);
    click_hotkey_panel.add(&click_hotkey_input);
    click_hotkey_panel.end();

    let mut record_hotkey_panel = Pack::new(0, 0, 200, 30, "");
    record_hotkey_panel.set_type(PackType::Horizontal);

    let record_label = Frame::new(0, 0, 50, 30, "Record");
    record_hotkey_panel.add(&record_label);

    let mut record_hotkey_input = Input::new(50, 0, 150, 30, "");
    record_hotkey_input.deactivate();
    record_hotkey_input.set_trigger(CallbackTrigger::Changed);
    record_hotkey_panel.add(&record_hotkey_input);
    record_hotkey_panel.end();

    panel.add(&click_hotkey_panel);
    panel.add(&record_hotkey_panel);
    panel.end();

    // hot_key_type:  1 for click hot key, 2 for record hot key
    register_hot_key(&mut click_hotkey_input, 1);
    register_hot_key(&mut record_hotkey_input, 2);

    panel
}