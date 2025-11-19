use dioxus::prelude::*;

use dioxus_i18n::prelude::{I18nConfig, use_init_i18n};
use sdk::app::components::AppProvider;
use sdk::app::storage::{use_local_storage, use_storage_is_enabled};
use unic_langid::{LanguageIdentifier, langid};

use crate::Routes;
use crate::constants::{FAVICON_ICO, STYLE_CSS};

#[component]
pub fn App() -> Element {
    let mut app_is_starting = use_signal(|| true);
    let storage_is_enabled = use_storage_is_enabled();
    let mut language_storage = use_local_storage::<LanguageIdentifier>("_language");

    let mut i18n = use_init_i18n(|| {
        I18nConfig::new(langid!("en"))
            .with_locale((langid!("en"), include_str!("../locales/en.ftl")))
            .with_locale((langid!("es"), include_str!("../locales/es.ftl")))
    });

    use_effect(move || {
        if storage_is_enabled()
            && let Some(language) = &*language_storage.peek()
        {
            i18n.set_language(language.clone());
        }
    });

    use_effect(move || {
        if storage_is_enabled() {
            language_storage.set(Some(i18n.language()));
        }
    });

    use_effect(move || {
        if storage_is_enabled() {
            app_is_starting.set(false);
        }
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
