use dioxus::prelude::*;

use super::IconSvg;
use crate::icons;

const STORAGE_KEY: &str = "mdbook-theme";

pub const THEME_SCRIPT: &str = r#"try {
    const saved = localStorage.getItem("mdbook-theme");
    if (saved === "light" || saved === "coal") {
        document.documentElement.dataset.theme = saved === "coal" ? "dark" : "light";
    }
} catch (e) {}"#;

#[derive(Clone, Copy, PartialEq)]
enum Theme {
    Auto,
    Light,
    Dark,
}

impl Theme {
    fn current() -> Self {
        match html()
            .and_then(|html| html.get_attribute("data-theme"))
            .as_deref()
        {
            Some("light") => Self::Light,
            Some("dark") => Self::Dark,
            _ => Self::Auto,
        }
    }

    fn next(self) -> Self {
        let (same, opposite) = if system_is_dark() {
            (Self::Dark, Self::Light)
        } else {
            (Self::Light, Self::Dark)
        };
        if self == Self::Auto {
            opposite
        } else if self == opposite {
            same
        } else {
            Self::Auto
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    fn apply(self) {
        let (data_theme, saved) = match self {
            Self::Auto => (None, None),
            Self::Light => (Some("light"), Some("light")),
            Self::Dark => (Some("dark"), Some("coal")),
        };
        if let Some(html) = html() {
            _ = match data_theme {
                Some(value) => html.set_attribute("data-theme", value),
                None => html.remove_attribute("data-theme"),
            };
        }
        if let Some(storage) =
            web_sys::window().and_then(|window| window.local_storage().ok().flatten())
        {
            _ = match saved {
                Some(value) => storage.set_item(STORAGE_KEY, value),
                None => storage.remove_item(STORAGE_KEY),
            };
        }
    }
}

fn html() -> Option<web_sys::Element> {
    web_sys::window()?.document()?.document_element()
}

fn system_is_dark() -> bool {
    web_sys::window()
        .and_then(|window| {
            window
                .match_media("(prefers-color-scheme: dark)")
                .ok()
                .flatten()
        })
        .is_some_and(|query| query.matches())
}

#[component]
pub fn ThemeButton() -> Element {
    let mut theme = use_signal(|| Theme::Auto);
    use_effect(move || theme.set(Theme::current()));
    let label = format!("Theme: {}", theme().name());

    rsx! {
        button {
            class: "btn btn-ghost btn-square",
            r#type: "button",
            title: "{label}",
            "aria-label": "{label}",
            onclick: move |_| {
                let next = theme().next();
                next.apply();
                theme.set(next);
            },
            IconSvg { icon: &icons::MONITOR, class: "hidden size-5 theme-auto:block" }
            IconSvg { icon: &icons::SUN, class: "hidden size-5 theme-light:block" }
            IconSvg { icon: &icons::MOON, class: "hidden size-5 theme-dark:block" }
        }
    }
}
