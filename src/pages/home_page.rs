use dioxus::prelude::*;

use sdk::app::components::{H2, Mango3Logo, PageTitle};

#[component]
pub fn FakeHomePage() -> Element {
    HomePage()
}

#[component]
pub fn HomePage() -> Element {
    rsx! {
        PageTitle { "Home" }

        section { class: "mt-4",
            div { class: "hero bg-base-200 p-4 rounded-2xl",
                div { class: "hero-content text-center",
                    div {
                        h1 { class: "h1 text-3xl", "Hi there!" }

                        div { class: "flex flex-wrap gap-2 mb-2 items-center justify-center",
                            span { class: "text-xl font-bold", "Welcome to " }

                            Mango3Logo { class: "h-5 w-auto" }
                        }

                        p { "We are a small group of people building some cool stuff." }
                    }
                }
            }
        }

        section { class: "mt-8",
            H2 { "Our Products" }

            div { class: "card card-border",
                div { class: "card-body flex-row items-center justify-between",
                    div {
                        h2 { class: "card-title h2", "Drive" }

                        p { "A cloud storage solution" }
                    }

                    a {
                        class: "btn btn-primary",
                        href: "https://drive.mango3.app",
                        target: "_blank",
                        "Join beta"
                    }
                }
            }
        }
    }
}
