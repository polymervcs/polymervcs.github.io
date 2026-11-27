use dioxus::prelude::*;

mod components;
mod icons;
mod views;

use views::{CatchAll, Downloads, Home, NotFound, Shell};

// Generated from /tailwind.css by dx on each build.
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const FAVICON: Asset = asset!("/assets/favicon.svg");
// The allo-font npm package, copied whole to /assets/allo-font/. Folder assets
// aren't hashed, so its CSS still finds the fonts at their relative paths.
// Install it with `npm install` before building.
const ALLO_FONT: Asset = asset!("/node_modules/allo-font", AssetOptions::folder());

// The newest PolymerVCS release: this crate's version in Cargo.toml, which the
// docs use too (book/version.py).
const LATEST_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Shell)]
        #[route("/")]
        Home {},
        #[route("/downloads")]
        Downloads {},
        // Pre-rendered and then moved to /404.html, which GitHub Pages serves
        // for any path that doesn't exist.
        #[route("/404")]
        NotFound {},
        // Client-side fallback for unknown links followed after the app loads.
        #[route("/:..segments")]
        CatchAll { segments: Vec<String> },
}

fn main() {
    #[cfg(feature = "server")]
    write_latest_json();

    dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            ServeConfig::builder()
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        .static_dir(public_dir())
                        // Keep the wasm and assets already in `public`.
                        .clear_cache(false),
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
}

#[cfg(feature = "server")]
fn public_dir() -> std::path::PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("public")
}

#[cfg(feature = "server")]
fn write_latest_json() {
    let path = public_dir().join("api/v1/releases/latest.json");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, format!("\"{LATEST_VERSION}\"\n")).unwrap();
}

#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect())
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", r#type: "image/svg+xml", href: FAVICON }
        document::Stylesheet { href: "{ALLO_FONT}/allo-friendly.css" }
        document::Link {
            rel: "preload",
            href: "{ALLO_FONT}/fonts/webfonts/AlloVariableFriendly.woff2",
            r#as: "font",
            r#type: "font/woff2",
            crossorigin: "anonymous",
        }
        document::Stylesheet { href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
