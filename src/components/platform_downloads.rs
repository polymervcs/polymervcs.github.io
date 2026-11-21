use dioxus::prelude::*;

use super::{CopyButton, IconSvg};
use crate::{LATEST_VERSION, icons};

const RELEASE_DOWNLOADS_URL: &str = "https://github.com/polymervcs/polymervcs/releases/download";

#[derive(PartialEq)]
pub struct Platform {
    pub name: &'static str,
    /// One logo for each system in `name`.
    pub icons: &'static [icons::Icon],
    pub downloads: &'static [Download],
}

#[derive(PartialEq)]
pub struct Download {
    pub label: &'static str,
    pub package: Package,
}

#[derive(PartialEq)]
pub enum Package {
    /// A file on the GitHub release, named with `<version>` in place of the
    /// version. It's shown as a button that downloads the file.
    File(&'static str),
    /// A container image, which isn't on the release.
    Image(&'static str),
}

#[component]
pub fn PlatformDownloads(platform: &'static Platform) -> Element {
    rsx! {
        section { class: "mt-12",
            h2 { class: "flex items-center gap-3 text-2xl font-bold",
                span { class: "flex shrink-0 gap-1.5",
                    for icon in platform.icons {
                        IconSvg { icon, class: "size-6" }
                    }
                }
                "{platform.name}"
            }
            div { class: "mt-4 overflow-x-auto rounded-box border border-base-300",
                table { class: "table",
                    tbody {
                        for download in platform.downloads {
                            tr { key: "{download.label}",
                                th { class: "font-medium sm:whitespace-nowrap", "{download.label}" }
                                td { class: "w-full text-right",
                                    match download.package {
                                        Package::File(file) => {
                                            let name = file.replace("<version>", LATEST_VERSION);
                                            rsx! {
                                                a {
                                                    class: "btn btn-soft btn-sm h-auto min-h-8 justify-start py-1.5 text-left font-normal",
                                                    href: "{RELEASE_DOWNLOADS_URL}/v{LATEST_VERSION}/{name}",
                                                    IconSvg { icon: &icons::DOWNLOAD, class: "size-4 shrink-0" }
                                                    code { class: "font-mono text-xs wrap-anywhere sm:text-sm md:whitespace-nowrap",
                                                        "{name}"
                                                    }
                                                }
                                            }
                                        }
                                        Package::Image(image) => rsx! {
                                            div { class: "flex items-center justify-end gap-2",
                                                code { class: "font-mono text-left text-xs wrap-anywhere sm:text-sm",
                                                    for part in image.split_inclusive('/') {
                                                        "{part}"
                                                        wbr {}
                                                    }
                                                }
                                                CopyButton { text: image, label: "Copy the image name" }
                                            }
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
