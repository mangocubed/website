use dioxus::prelude::*;

mod app;
mod constants;
mod layout;
mod pages;

use app::App;
use layout::Layout;
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
