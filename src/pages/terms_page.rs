use dioxus::prelude::*;

use crate::components::PageTitle;

#[component]
pub fn TermsPage() -> Element {
    rsx! {
        PageTitle { "Terms of Service" }

        h1 { class: "h1", "Terms of Service" }
    }
}
