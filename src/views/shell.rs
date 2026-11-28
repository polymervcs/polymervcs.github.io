use dioxus::prelude::*;

use crate::components::ThemeButton;
use crate::Route;

#[component]
pub fn Shell() -> Element {
    rsx! {
        div { class: "relative flex min-h-screen flex-col bg-base-100 text-base-content",
            // There's no header, so the theme button sits in the top corner.
            div { class: "absolute top-2 right-2", ThemeButton {} }
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
