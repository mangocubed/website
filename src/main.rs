use dioxus::prelude::*;

mod components;
mod pages;

use pages::{Page, PageSlug};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
enum Routes {
    #[layout(Layout)]
        #[route("/#:slug")]
        Page { slug: PageSlug },
}

impl Routes {
    fn home() -> Self {
        Self::Page { slug: PageSlug::Home }
    }

    fn privacy() -> Self {
        Self::Page {
            slug: PageSlug::Privacy,
        }
    }

    fn terms() -> Self {
        Self::Page { slug: PageSlug::Terms }
    }
}

const FAVICON_ICO: Asset = asset!("assets/favicon.ico");
const ICON_SVG: Asset = asset!("assets/icon.svg");
const LOGO_SVG: Asset = asset!("assets/logo.svg");
const STYLE_CSS: Asset = asset!("assets/style.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON_ICO }
        document::Link { rel: "stylesheet", href: STYLE_CSS }

        Router::<Routes> {}
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        div { class: "navbar bg-base-300",
            div { class: "navbar-start",
                Link { class: "p-2 font-bold", to: Routes::home(),
                    img { class: "h-[36px] sm:hidden", src: ICON_SVG }
                    img { class: "h-[36px] max-sm:hidden", src: LOGO_SVG }
                }
            }

            div { class: "navbar-right",
                ul { class: "menu menu-horizontal",
                    li {
                        Link { to: Routes::home(), "Home" }
                    }

                    li {
                        Link { to: Routes::terms(), "Terms of Service" }
                    }

                    li {
                        Link { to: Routes::privacy(), "Privacy Policy" }
                    }
                }
            }
        }

        main { class: "main", Outlet::<Routes> {} }
    }
}
