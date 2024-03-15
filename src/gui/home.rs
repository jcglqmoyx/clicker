use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use fltk::window::Window;

use crate::gui::components::additional_settings_panel::additional_settings_panel;
use crate::gui::components::count_click_panel::count_click_panel;
use crate::gui::components::help_and_about_panel::help_and_about_panel;
use crate::gui::components::hotkey_settings_panel::hotkey_settings_panel;
use crate::gui::components::mouse_panel::mouse_panel;
use crate::gui::components::smart_click_settings_panel::smart_click_settings_panel;
use crate::gui::components::status_panel::status_panel;
use crate::gui::components::time_internal_panel::time_interval_panel;
use crate::utils::border::draw_border;

pub unsafe fn window() -> Window {
    let mut window = Window::new(100, 100, 550, 550, "Meow Auto Clicker 1.0");
    window.make_resizable(true);

    let mouse_panel = mouse_panel();
    let count_click_panel = count_click_panel();
    let time_interval_panel = time_interval_panel();
    let additional_settings_panel = additional_settings_panel();
    let hotkey_settings_panel = hotkey_settings_panel();
    let smart_click_settings_panel = smart_click_settings_panel();
    let help_and_about_panel = help_and_about_panel();
    let status_panel = status_panel();

    window.add(&mouse_panel);
    window.add(&count_click_panel);
    window.add(&time_interval_panel);
    window.add(&additional_settings_panel);
    window.add(&hotkey_settings_panel);
    window.add(&smart_click_settings_panel);
    window.add(&help_and_about_panel);
    window.add(&status_panel);

    window.draw(move |_| {
        draw_border(&mouse_panel, 10, 10, 10, 20);
        draw_border(&count_click_panel, 10, 10, 20, 20);
        draw_border(&time_interval_panel, 10, 0, 10, 20);
        draw_border(&additional_settings_panel, 10, 0, 10, 10);
        draw_border(&hotkey_settings_panel, 10, 0, 20, 10);
        draw_border(&smart_click_settings_panel, 10, 0, 10, 10);
        draw_border(&help_and_about_panel, 0, 0, 0, 0);
        draw_border(&status_panel, 0, 0, 0, 0);
    });

    window.end();
    window.show();
    window
}