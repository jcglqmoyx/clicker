use fltk::button::Button;
use fltk::group::Pack;
use fltk::prelude::{GroupExt, WidgetBase};

pub fn status_panel() -> Pack {
    let mut panel = Pack::new(40, 490, 470, 30, "");
    let status_button = Button::new(50, 490, 470, 30, "status");
    panel.add(&status_button);
    panel.end();
    panel
}
