use fltk::app::Sender;
use fltk::input::Input;
use fltk::prelude::{GroupExt, InputExt, WidgetBase, WidgetExt};
use fltk::window::Window;

use crate::state::{UiMsg, WindowWidgets};

use crate::gui::components::additional_settings_panel::additional_settings_panel;
use crate::gui::components::count_click_panel::count_click_panel;
use crate::gui::components::help_and_about_panel::help_and_about_panel;
use crate::gui::components::hotkey_settings_panel::hotkey_settings_panel;
use crate::gui::components::mouse_panel::mouse_panel;
use crate::gui::components::smart_click_settings_panel::smart_click_settings_panel;
use crate::gui::components::status_panel::status_panel;
use crate::gui::components::time_internal_panel::time_interval_panel;
use crate::utils::border::draw_border;

/// Build the main window and return it together with the widget handles needed
/// to apply [`UiMsg`] updates on the main thread.
pub fn window(ui: Sender<UiMsg>) -> (Window, WindowWidgets) {
    let mut window = Window::new(100, 100, 550, 550, "Meow Auto Clicker 3.0");
    window.make_resizable(true);

    let mouse_panel = mouse_panel();
    let count_click_panel = count_click_panel();
    let time_interval_panel = time_interval_panel();
    let (additional_panel, x_coord_frame, y_coord_frame) = additional_settings_panel(ui.clone());
    let hotkey_settings_panel = hotkey_settings_panel();
    let status_button = status_panel();

    let mut count_record_input = Input::new(50, 300, 60, 0, "");
    count_record_input.deactivate();

    let smart_click_settings_panel =
        smart_click_settings_panel(count_record_input.clone());

    let help_and_about_panel = help_and_about_panel();

    window.add(&mouse_panel);
    window.add(&count_click_panel);
    window.add(&time_interval_panel);
    window.add(&additional_panel);
    window.add(&hotkey_settings_panel);
    window.add(&status_button);
    window.add(&smart_click_settings_panel);
    window.add(&help_and_about_panel);

    let widgets = WindowWidgets {
        status_button: status_button.clone(),
        count_record_input,
        x_coord_frame,
        y_coord_frame,
    };

    window.draw(move |_| {
        draw_border(&mouse_panel, 10, 10, 10, 20);
        draw_border(&count_click_panel, 10, 10, 20, 20);
        draw_border(&time_interval_panel, 10, 0, 10, 20);
        draw_border(&additional_panel, 10, 0, 10, 10);
        draw_border(&hotkey_settings_panel, 10, 0, 20, 10);
        draw_border(&smart_click_settings_panel, 10, 0, 10, 10);
        draw_border(&help_and_about_panel, 0, 0, 0, 0);
        draw_border(&status_button, 0, 0, 0, 0);
    });

    window.end();
    window.show();
    (window, widgets)
}

/// Apply a message sent from a worker thread to the relevant widget. Must only
/// be called on the main GUI thread.
pub(crate) fn apply_ui(msg: &UiMsg, w: &mut WindowWidgets) {
    match msg {
        UiMsg::Status { label, color } => {
            w.status_button.set_label(label);
            w.status_button.set_label_color(*color);
        }
        UiMsg::RecordCount(n) => {
            w.count_record_input.set_value(&n.to_string());
        }
        UiMsg::Coordinates { x, y } => {
            w.x_coord_frame.set_label(&format!("X: {}", x));
            w.y_coord_frame.set_label(&format!("Y: {}", y));
        }
    }
}
