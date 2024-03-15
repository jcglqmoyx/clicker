use fltk::button::Button;
use fltk::button::CheckButton;
use fltk::frame::Frame;
use fltk::group::{Pack, PackType};
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};

use crate::global::click::COUNT_RECORD_INPUT;
use crate::register::record::{on_clear_record_button_clicked, on_save_record_button_clicked};
use crate::register::smart_click::toggle_click_mode;

pub unsafe fn smart_click_settings_panel() -> Pack {
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
    COUNT_RECORD_INPUT.deactivate();
    record_count_panel.add(&records_label);
    record_count_panel.end();

    let mut record_operation_panel = Pack::new(50, 400, 100, 30, "");
    record_operation_panel.set_type(PackType::Horizontal);
    record_operation_panel.set_spacing(5);
    let mut clear_record_button = Button::new(50, 400, 60, 20, "Clear");
    on_clear_record_button_clicked(&mut clear_record_button);
    let mut save_record_button = Button::new(50, 450, 60, 20, "Save");
    on_save_record_button_clicked(&mut save_record_button);

    let load_button = Button::new(50, 500, 60, 20, "Load");
    record_operation_panel.add(&clear_record_button);
    record_operation_panel.add(&save_record_button);
    record_operation_panel.add(&load_button);
    record_operation_panel.end();

    panel.add(&smart_click_panel);
    panel.add(&record_count_panel);
    panel.add(&record_operation_panel);
    panel.end();

    panel
}