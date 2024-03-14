use fltk::button::Button;
use fltk::button::CheckButton;
use fltk::frame::Frame;
use fltk::group::{Pack, PackType};
use fltk::input::Input;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};

use crate::register::smart_click::toggle_click_mode;

pub fn smart_click_settings_panel() -> Pack {
    let mut panel = Pack::new(50, 350, 200, 120, "");
    panel.set_spacing(10);
    panel.set_type(PackType::Vertical);

    let mut smart_click_panel = Pack::new(50, 300, 100, 30, "");
    let mut smart_click_button = CheckButton::new(50, 300, 100, 30, "Smart click");
    toggle_click_mode(&mut smart_click_button);
    smart_click_panel.add(&smart_click_button);
    smart_click_panel.end();

    let mut record_count_panel = Pack::new(50, 330, 100, 30, "");
    record_count_panel.set_type(PackType::Horizontal);
    record_count_panel.set_spacing(5);
    let records_label = Frame::new(50, 330, 60, 0, "Records");
    let mut count_record_input = Input::new(50, 300, 60, 0, "");
    count_record_input.deactivate();
    let edit_button = Button::new(50, 400, 60, 0, "Edit");
    record_count_panel.add(&records_label);
    record_count_panel.add(&count_record_input);
    record_count_panel.add(&edit_button);
    record_count_panel.end();

    let mut record_operation_panel = Pack::new(50, 400, 100, 30, "");
    record_operation_panel.set_type(PackType::Horizontal);
    record_operation_panel.set_spacing(5);
    let clear_button = Button::new(50, 400, 60, 20, "Clear");
    let save_button = Button::new(50, 450, 60, 20, "Save");
    let load_button = Button::new(50, 500, 60, 20, "Load");
    record_operation_panel.add(&clear_button);
    record_operation_panel.add(&save_button);
    record_operation_panel.add(&load_button);
    record_operation_panel.end();

    panel.add(&smart_click_panel);
    panel.add(&record_count_panel);
    panel.add(&record_operation_panel);
    panel.end();

    panel
}