use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Title { "PolymerVCS" }
        section { class: "hero flex-1 py-24",
            div { class: "hero-content text-center",
                div { class: "max-w-md",
                    h1 { class: "text-5xl font-bold", "PolymerVCS" }
                    p { class: "py-6 text-lg text-base-content/70",
                        "An upcoming Version Control System for game development, and other projects with text and large binary files."
                    }
                    div { class: "flex justify-center gap-3",
                        Link { class: "btn btn-primary", to: Route::Downloads {}, "Download" }
                        // The docs are the mdBook in book/, not a route, so this is a plain link that loads them.
                        a { class: "btn", href: "/docs/", "Documentation" }
                    }
                }
            }
        }
    }
}
