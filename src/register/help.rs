use fltk::prelude::WidgetExt;
use locale_config::Locale;

pub(crate) fn on_help_button_clicked(button: &mut fltk::button::Button) {
    button.set_callback(move |_| {
        let current_locale = Locale::current();

        println!("Current locale: {:?}", current_locale.as_ref());
        // let language = current_locale
        // println!("Current language: {}", language);
        // let _ = webbrowser::open("http://www.rust-lang.org");
    });
}

pub(crate) fn on_contact_author_button_clicked(button: &mut fltk::button::Button) {
    button.set_callback(move |_| {
        let _ = webbrowser::open("https://linktr.ee/jcglqmoyx");
    });
}