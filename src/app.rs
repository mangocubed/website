use dioxus::prelude::*;

use dioxus_i18n::prelude::{I18nConfig, use_init_i18n};
use sdk::app::components::AppProvider;
use sdk::app::storage::{LocalStorage, StorageBacking};
use unic_langid::langid;

use crate::Routes;
use crate::constants::{FAVICON_ICO, KEY_LANGUAGE, STYLE_CSS};

#[component]
pub fn App() -> Element {
    let mut app_is_starting = use_signal(|| true);

    let mut i18n = use_init_i18n(|| {
        I18nConfig::new(langid!("en"))
            .with_locale((langid!("en"), include_str!("../locales/en.ftl")))
            .with_locale((langid!("es"), include_str!("../locales/es.ftl")))
    });

    use_effect(move || {
        i18n.set_language(LocalStorage::get(&KEY_LANGUAGE.to_owned()).unwrap_or_else(|| langid!("en")));
        app_is_starting.set(false);
    });

    use_effect(move || {
        LocalStorage::set(KEY_LANGUAGE.to_owned(), &i18n.language());
    });

    rsx! {
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1, maximum-scale=1, user-scalable=0",
        }
        document::Link { rel: "icon", href: FAVICON_ICO }
        document::Link { rel: "stylesheet", href: STYLE_CSS }

        AppProvider { is_starting: app_is_starting, Router::<Routes> {} }
    }
}
