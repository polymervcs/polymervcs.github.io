use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::JsFuture;

use super::IconSvg;
use crate::icons;

#[component]
pub fn CopyButton(text: &'static str, label: &'static str) -> Element {
    let mut copied = use_signal(|| false);
    rsx! {
        button {
            class: "btn btn-soft btn-sm btn-square",
            r#type: "button",
            title: label,
            "aria-label": label,
            onclick: move |_| {
                // Safari only lets a page write to the clipboard while it's
                // handling a click, so start the write here and wait for it in
                // a task.
                let Some(window) = web_sys::window() else {
                    return;
                };
                let write = JsFuture::from(window.navigator().clipboard().write_text(text));
                spawn(async move {
                    if write.await.is_err() {
                        return;
                    }
                    copied.set(true);
                    TimeoutFuture::new(2_000).await;
                    copied.set(false);
                });
            },
            IconSvg {
                icon: if copied() { &icons::CHECK } else { &icons::COPY },
                class: "size-4",
            }
        }
        span { class: "sr-only", "aria-live": "polite",
            if copied() {
                "Copied"
            }
        }
    }
}
