use dioxus::prelude::*;

use sdk::app::components::{Mango3Logo, PageTitle};

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
                        h1 { class: "h1 text-4xl", "Hi there!" }

                        div { class: "flex flex-wrap gap-2 mb-2 items-center justify-center",
                            span { class: "text-2xl font-bold", "Welcome to " }

                            Mango3Logo { class: "h-6 w-auto" }
                        }

                        p { class: "text-xl",
                            "We are a small group of people building some cool stuff."
                        }
                    }
                }
            }
        }

        section { class: "mt-8",
            h2 { class: "h2 text-3xl", "Our Products" }

            div { class: "card card-border",
                div { class: "card-body flex-row items-center justify-between",
                    div {
                        h3 { class: "h3 card-title text-2xl", "Drive" }

                        p { class: "text-xl", "A cloud storage solution" }
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
