use dioxus::prelude::*;
use dioxus_i18n::prelude::i18n;

use sdk::app::components::{Brand, Footer, Navbar, NavbarStart};
use sdk::app::icons::{CheckMini, ChevronUpMini};
use sdk::constants::COPYRIGHT;

use crate::Routes;
use crate::constants::{LANGUAGE_NAMES, SOURCE_CODE_URL};

#[component]
pub fn Layout() -> Element {
    let mut i18n = i18n();
    let language_name = use_memo(move || *LANGUAGE_NAMES.get(&i18n.language()).unwrap());

    let language_names = || {
        let mut names = LANGUAGE_NAMES.iter().collect::<Vec<_>>();
        names.sort_by_key(|(_, name)| *name);
        names
    };

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

                nav {
                    div { class: "dropdown dropdown-top",
                        button {
                            class: "btn btn-outline btn-accent w-36 justify-between normal-case",
                            tabindex: "0",
                            {language_name()}
                            ChevronUpMini {}
                        }

                        ul {
                            class: "dropdown-content menu w-36 p-2 mb-2",
                            tabindex: "0",
                            for (id , name) in language_names() {
                                li { class: if i18n.language() == *id { "menu-active" },
                                    a {
                                        class: "flex justify-between",
                                        onclick: move |_| i18n.set_language(id.clone()),
                                        {*name}
                                        if i18n.language() == *id {
                                            CheckMini {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
