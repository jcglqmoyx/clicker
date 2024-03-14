use fltk::button::Button;
use fltk::group::{Pack, PackType};
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};

pub fn help_and_about_panel() -> Pack {
    let mut panel = Pack::new(290, 350, 220, 120, "");
    panel.set_spacing(10);
    panel.set_type(PackType::Vertical);

    let help_button = Button::new(300, 350, 220, 55, "Help");
    let contact_author_button = Button::new(300, 420, 220, 55, "Contact author");

    panel.add(&help_button);
    panel.add(&contact_author_button);
    panel.end();

    panel
}
