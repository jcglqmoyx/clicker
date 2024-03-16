use fltk::button::Button;
use fltk::group::{Pack, PackType};
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use crate::register::help::{on_contact_author_button_clicked, on_help_button_clicked};

pub fn help_and_about_panel() -> Pack {
    let mut panel = Pack::new(290, 350, 220, 120, "");
    panel.set_spacing(10);
    panel.set_type(PackType::Vertical);

    let mut help_button = Button::new(300, 350, 220, 55, "Help");
    let mut contact_author_button = Button::new(300, 420, 220, 55, "Contact author");
    on_help_button_clicked(&mut help_button);
    on_contact_author_button_clicked(&mut contact_author_button);

    panel.add(&help_button);
    panel.add(&contact_author_button);
    panel.end();

    panel
}
