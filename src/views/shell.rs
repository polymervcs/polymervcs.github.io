use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Shell() -> Element {
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
