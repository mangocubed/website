use dioxus::prelude::*;

use crate::components::PageTitle;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        PageTitle { "Home" }
    }
}
