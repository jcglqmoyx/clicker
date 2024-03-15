use fltk::button::Button;
use fltk::button::CheckButton;
use fltk::frame::Frame;
use fltk::group::{Pack, PackType};
use fltk::input::Input;
use fltk::prelude::{GroupExt, InputExt, WidgetBase, WidgetExt};
use fltk::window::Window;

use crate::global::click::COUNT_RECORD_INPUT;
use crate::register::record::on_clear_record_button_clicked;
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


    save_record_button.set_callback(move |_| {
        let mut save_window = Window::new(200, 200, 300, 200, "Save");
        let mut title_input = Input::new(20, 20, 260, 30, "Title:");
        let mut description_input = Input::new(20, 60, 260, 30, "Description:");
        let mut content_input = Input::new(20, 100, 260, 30, "Content:");
        let mut save_button = Button::new(80, 150, 60, 30, "Save");
        let mut cancel_button = Button::new(160, 150, 60, 30, "Cancel");

        save_button.set_callback({
            let mut save_window = save_window.clone();
            move |_| {
                let title = title_input.value();
                let description = description_input.value();
                let content = content_input.value();

                // save_to_database(&title, &description, &content);

                save_window.hide();
            }
        });

        cancel_button.set_callback({
            let mut save_window = save_window.clone();
            move |_| {
                save_window.hide();
            }
        });

        save_window.end();
        save_window.show();
    });

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