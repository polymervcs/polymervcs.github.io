use dioxus::prelude::*;

use crate::icons::Icon;

#[component]
pub fn IconSvg(icon: &'static Icon, class: &'static str) -> Element {
    rsx! {
        svg {
            class,
            view_box: icon.view_box,
            fill: "currentColor",
            "aria-hidden": "true",
            path { d: icon.path }
        }
    }
}
