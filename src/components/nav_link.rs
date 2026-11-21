use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn NavLink(to: Route, children: Element) -> Element {
    let current = use_route::<Route>();
    let class = if current == to { "menu-active" } else { "" };
    rsx! {
        Link { class, to, {children} }
    }
}
