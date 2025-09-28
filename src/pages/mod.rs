use std::{fmt::Display, str::FromStr};

use dioxus::prelude::*;

mod home_page;
mod privacy_page;
mod terms_page;

use home_page::HomePage;
use privacy_page::PrivacyPage;
use terms_page::TermsPage;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum PageSlug {
    #[default]
    Home,
    Privacy,
    Terms,
    NotFound,
}

pub struct PageSlugError;

impl Display for PageSlugError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid page slug")
    }
}

impl Display for PageSlug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageSlug::Home => write!(f, ""),
            PageSlug::Privacy => write!(f, "privacy"),
            PageSlug::Terms => write!(f, "terms"),
            PageSlug::NotFound => write!(f, "not-found"),
        }
    }
}

impl FromStr for PageSlug {
    type Err = PageSlugError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(PageSlug::Home),
            "privacy" => Ok(PageSlug::Privacy),
            "terms" => Ok(PageSlug::Terms),
            _ => Ok(PageSlug::NotFound),
        }
    }
}

#[component]
pub fn Page(slug: ReadSignal<PageSlug>) -> Element {
    match slug() {
        PageSlug::Home => HomePage(),
        PageSlug::Privacy => PrivacyPage(),
        PageSlug::Terms => TermsPage(),
        PageSlug::NotFound => rsx! {
            h1 { class: "h1", "Page not found" }
        },
    }
}
