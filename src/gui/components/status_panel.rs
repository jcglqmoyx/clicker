use fltk::enums::{Color};
use fltk::group::Pack;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};

use crate::global::status::STATUS_BUTTON;

pub unsafe fn status_panel() -> Pack {
    let mut panel = Pack::new(40, 490, 470, 30, "");
    STATUS_BUTTON.set_label_color(Color::Red);
    panel.end();
    panel
}
