use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn NotFound() -> Element {
    rsx! { NotFoundView {} }
}

#[component]
pub fn CatchAll(segments: Vec<String>) -> Element {
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
