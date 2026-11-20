use dioxus::prelude::*;

// Generated from /tailwind.css by dx on each build.
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const FAVICON: Asset = asset!("/assets/favicon.svg");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Shell)]
        #[route("/")]
        Home {},
        // Pre-rendered and then moved to /404.html, which GitHub Pages serves
        // for any path that doesn't exist.
        #[route("/404")]
        NotFound {},
        // Client-side fallback for unknown links followed after the app loads.
        #[route("/:..segments")]
        CatchAll { segments: Vec<String> },
}

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            ServeConfig::builder()
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        // `dx` writes the pre-rendered pages next to the wasm bundle
                        // in the `public` folder beside the server binary.
                        .static_dir(
                            std::env::current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public"),
                        )
                        // Keep the wasm and assets already in `public`.
                        .clear_cache(false),
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
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
        document::Stylesheet { href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Shell() -> Element {
    rsx! {
        div { class: "flex min-h-screen flex-col bg-base-100 text-base-content",
            // header { class: "navbar border-b border-base-300 bg-base-100",
            //     div { class: "flex-1",
            //         Link { class: "btn btn-ghost text-xl", to: Route::Home {}, "PolymerVCS" }
            //     }
            //     nav { class: "flex-none",
            //         ul { class: "menu menu-horizontal px-1",
            //             li { NavLink { to: Route::Home {}, "Home" } }
            //         }
            //     }
            // }
            main { class: "flex flex-1 flex-col", Outlet::<Route> {} }
            footer { class: "footer footer-center bg-base-200 p-6 text-base-content/70",
                aside { p { "© PolymerVCS Software Corporation" } }
            }
        }
    }
}

/// A daisyUI menu link marked active on its page. `Link`'s own active check
/// compares raw URLs, which misses `/about/`, the form GitHub Pages redirects
/// `/about` to. Comparing parsed routes handles both.
#[component]
fn NavLink(to: Route, children: Element) -> Element {
    let current = use_route::<Route>();
    let class = if current == to { "menu-active" } else { "" };
    rsx! {
        Link { class, to, {children} }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        document::Title { "PolymerVCS" }
        section { class: "hero flex-1 py-24",
            div { class: "hero-content text-center",
                div { class: "max-w-md",
                    h1 { class: "text-5xl font-bold", "PolymerVCS" }
                    p { class: "py-6 text-lg text-base-content/70",
                        "An upcoming Version Control System for game development, and other projects with text and large binary files."
                    }
                }
            }
        }
    }
}

#[component]
fn NotFound() -> Element {
    rsx! { NotFoundView {} }
}

#[component]
fn CatchAll(segments: Vec<String>) -> Element {
    let _ = segments;
    rsx! { NotFoundView {} }
}

#[component]
fn NotFoundView() -> Element {
    rsx! {
        document::Title { "Page not found · PolymerVCS" }
        section { class: "hero flex-1 py-24",
            div { class: "hero-content text-center",
                div { class: "max-w-md",
                    h1 { class: "text-4xl font-bold", "Page not found" }
                    p { class: "py-6 text-base-content/70", "There's nothing at this address." }
                    Link { class: "btn btn-primary", to: Route::Home {}, "Back to the home page" }
                }
            }
        }
    }
}
