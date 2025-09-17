use dioxus::prelude::*;

mod components;
mod pages;

use pages::{HomePage, PrivacyPage, TermsPage};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
enum Routes {
    #[layout(Layout)]
        #[route("/")]
        HomePage {},
        #[route("/privacy")]
        PrivacyPage {},
        #[route("/terms")]
        TermsPage {},
}

impl Routes {
    fn home() -> Self {
        Self::HomePage {}
    }

    fn privacy() -> Self {
        Self::PrivacyPage {}
    }

    fn terms() -> Self {
        Self::TermsPage {}
    }
}

const FAVICON_ICO: Asset = asset!("assets/favicon.ico");
const LOGO_SVG: Asset = asset!("assets/logo.svg");
const STYLE_CSS: Asset = asset!("assets/style.css");

fn main() {
    dioxus::LaunchBuilder::new()
        // Set the server config only if we are building the server target
        .with_cfg(server_only! {
            ServeConfig::builder()
                // Enable incremental rendering
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        // Store static files in the public directory where other static assets like wasm are stored
                        .static_dir(
                            std::env::current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public")
                        )
                        // Don't clear the public folder on every build. The public folder has other files including the wasm
                        // binary and static assets required for the app to run
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
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
                    img { class: "h-8", src: LOGO_SVG }
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
