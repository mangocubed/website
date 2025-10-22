use dioxus::prelude::*;

use sdk::app::components::{Brand, Footer, Navbar, NavbarStart};
use sdk::constants::COPYRIGHT;

use crate::Routes;
use crate::constants::SOURCE_CODE_URL;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            Navbar {
                NavbarStart {
                    Link { to: Routes::home(), Brand {} }
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
