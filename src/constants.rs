use std::collections::HashMap;
use std::sync::LazyLock;

use dioxus::prelude::{Asset, asset, manganis};
use unic_langid::{LanguageIdentifier, langid};

pub const FAVICON_ICO: Asset = asset!("assets/favicon.ico");
pub const STYLE_CSS: Asset = asset!("assets/style.css");

pub const KEY_LANGUAGE: &str = "_website_language";

pub static LANGUAGE_NAMES: LazyLock<HashMap<LanguageIdentifier, &str>> = LazyLock::new(|| {
    let mut language_names = HashMap::new();
    language_names.insert(langid!("en"), "English");
    language_names.insert(langid!("es"), "Español");
    language_names
});

pub static SOURCE_CODE_URL: LazyLock<String> =
    LazyLock::new(|| format!("{}/tree/{}", env!("CARGO_PKG_REPOSITORY"), env!("GIT_REV_SHORT")));
