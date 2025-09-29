use dioxus::prelude::*;

use sdk::components::Footer;

mod constants;
mod pages;

use constants::{COPYRIGHT, SOURCE_CODE_URL};
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
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1, maximum-scale=1, user-scalable=0",
        }
        document::Link { rel: "icon", href: FAVICON_ICO }
        document::Link { rel: "stylesheet", href: STYLE_CSS }

        Router::<Routes> {}
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
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
                            Link { to: Routes::terms(), "Terms of Service" }
                        }

                        li {
                            Link { to: Routes::privacy(), "Privacy Policy" }
                        }
                    }
                }
            }

            main { class: "main grow", Outlet::<Routes> {} }

            Footer {
                aside { class: "opacity-75",
                    p {
                        "Version: "
                        {env!("CARGO_PKG_VERSION")}
                        " ("
                        {env!("GIT_REV_SHORT")}
                        ")"
                    }

                    p {
                        "Built on: "
                        {env!("BUILD_DATETIME")}
                    }

                    p { {COPYRIGHT} }
                }

                nav {
                    Link { to: Routes::terms(), "Terms of Service" }

                    Link { to: Routes::privacy(), "Privacy Policy" }

                    a {
                        class: "link",
                        href: SOURCE_CODE_URL.clone(),
                        target: "_blank",
                        "Source code"
                    }
                }
            }
        }
    }
}
