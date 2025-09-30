use dioxus::prelude::*;

use sdk::components::{AppProvider, Brand, Footer, LoadingOverlay, Navbar, NavbarEnd, NavbarStart};

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
const STYLE_CSS: Asset = asset!("assets/style.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut app_is_loading = use_signal(|| true);

    use_effect(move || app_is_loading.set(false));

    rsx! {
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1, maximum-scale=1, user-scalable=0",
        }
        document::Link { rel: "icon", href: FAVICON_ICO }
        document::Link { rel: "stylesheet", href: STYLE_CSS }

        AppProvider { Router::<Routes> {} }

        LoadingOverlay { is_visible: app_is_loading }
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            Navbar {
                NavbarStart {
                    Link { to: Routes::home(), Brand {} }
                }

                NavbarEnd {
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
