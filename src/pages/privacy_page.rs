use dioxus::prelude::*;

use crate::components::PageTitle;

#[component]
pub fn PrivacyPage() -> Element {
    rsx! {
        PageTitle { "Privacy Policy" }

        h1 { class: "h1", "Privacy Policy" }
    }
}
