use fltk::prelude::WidgetExt;
use locale_config::Locale;

pub(crate) fn on_help_button_clicked(button: &mut fltk::button::Button) {
    button.set_callback(move |_| {
        let current_locale = Locale::current();
        let language = current_locale.as_ref();
        if language.contains("zh-CH") {
            let _ = webbrowser::open("https://zhuanlan.zhihu.com/p/687422006");
        } else {
            let _ = webbrowser::open("https://github.com/jcglqmoyx/clicker");
        }
    });
}

pub(crate) fn on_contact_author_button_clicked(button: &mut fltk::button::Button) {
    button.set_callback(move |_| {
        let _ = webbrowser::open("https://linktr.ee/jcglqmoyx");
    });
}