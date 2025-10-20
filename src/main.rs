use dioxus::prelude::*;

use sdk::app::components::{AppProvider, Brand, Footer, Navbar, NavbarEnd, NavbarStart};
use sdk::constants::COPYRIGHT;

mod constants;
mod pages;

use constants::SOURCE_CODE_URL;
use pages::{FakeHomePage, HomePage, PrivacyPage, TermsPage};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
enum Routes {
    #[layout(Layout)]
        #[route("/")]
        FakeHomePage,
        #[route("/home")]
        HomePage,
        #[route("/privacy")]
        PrivacyPage,
        #[route("/terms")]
        TermsPage,
}

impl Routes {
    fn home() -> Self {
        Self::FakeHomePage
    }

    fn privacy() -> Self {
        Self::PrivacyPage
    }

    fn terms() -> Self {
        Self::TermsPage
    }
}

const FAVICON_ICO: Asset = asset!("assets/favicon.ico");
const STYLE_CSS: Asset = asset!("assets/style.css");

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            ServeConfig::builder()
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        .static_dir(
                            std::env::current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public")
                        )
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
}

#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Routes::static_routes().iter().map(ToString::to_string).collect())
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

        AppProvider { is_starting: app_is_loading, Router::<Routes> {} }
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
