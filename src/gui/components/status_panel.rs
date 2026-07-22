use fltk::button::Button;
use fltk::enums::{Align, Color};
use fltk::prelude::{WidgetBase, WidgetExt};

pub fn status_panel() -> Button {
    // Spans the full width of the bottom row (matching the original layout),
    // with the status text centered like a status bar.
    let mut status_button = Button::new(40, 490, 470, 30, "Stopped clicking");
    status_button.set_label_color(Color::Red);
    status_button.set_align(Align::Inside | Align::Center);
    status_button
}
