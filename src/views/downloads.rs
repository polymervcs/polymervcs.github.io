use dioxus::prelude::*;

use crate::components::{Download, IconSvg, Package, Platform, PlatformDownloads};
use crate::icons;

const APP_AND_CLI: &str = "GUI + CMD clients";
const APP: &str = "GUI";
const CLI: &str = "CMD";
const SERVER: &str = "Server";

const PLATFORMS: &[Platform] = &[
    Platform {
        name: "Debian, Ubuntu, Linux Mint",
        icons: &[icons::DEBIAN, icons::UBUNTU, icons::LINUX_MINT],
        downloads: &[
            Download {
                label: APP,
                package: Package::File("polymervcs-gui_<version>_amd64.deb"),
            },
            Download {
                label: CLI,
                package: Package::File("polymervcs-cmd_<version>_amd64.deb"),
            },
            Download {
                label: SERVER,
                package: Package::File("polymervcs-server_<version>_amd64.deb"),
            },
        ],
    },
    Platform {
        name: "Fedora, openSUSE",
        icons: &[icons::FEDORA, icons::OPENSUSE],
        downloads: &[
            Download {
                label: APP,
                package: Package::File("polymervcs-gui-<version>.x86_64.rpm"),
            },
            Download {
                label: CLI,
                package: Package::File("polymervcs-cmd-<version>.x86_64.rpm"),
            },
            Download {
                label: SERVER,
                package: Package::File("polymervcs-server-<version>.x86_64.rpm"),
            },
        ],
    },
    Platform {
        name: "Arch Linux",
        icons: &[icons::ARCH_LINUX],
        downloads: &[
            Download {
                label: APP,
                package: Package::File("polymervcs-gui-<version>-x86_64.pkg.tar.zst"),
            },
            Download {
                label: CLI,
                package: Package::File("polymervcs-cmd-<version>-x86_64.pkg.tar.zst"),
            },
            Download {
                label: SERVER,
                package: Package::File("polymervcs-server-<version>-x86_64.pkg.tar.zst"),
            },
        ],
    },
    Platform {
        name: "macOS",
        icons: &[icons::APPLE],
        downloads: &[
            Download {
                label: APP_AND_CLI,
                package: Package::File("polymervcs-<version>-universal.pkg"),
            },
            Download {
                label: SERVER,
                package: Package::File("polymervcs-server-<version>-universal.pkg"),
            },
        ],
    },
    Platform {
        name: "Windows",
        icons: &[icons::WINDOWS],
        downloads: &[
            Download {
                label: APP_AND_CLI,
                package: Package::File("polymervcs-<version>-x64-setup.exe"),
            },
            Download {
                label: SERVER,
                package: Package::File("polymervcs-server-<version>-x64-setup.exe"),
            },
        ],
    },
    Platform {
        name: "Docker",
        icons: &[icons::DOCKER],
        downloads: &[Download {
            label: SERVER,
            package: Package::Image("ghcr.io/polymervcs/polymervcs"),
        }],
    },
];

#[component]
pub fn Downloads() -> Element {
    rsx! {
        document::Title { "Downloads · PolymerVCS" }
        section { class: "mx-auto w-full max-w-3xl px-4 py-16 lg:max-w-5xl",
            h1 { class: "text-4xl font-bold", "Download PolymerVCS" }

            div { class: "alert alert-warning mt-6",
                IconSvg { icon: &icons::WARNING, class: "size-6" }
                p {
                    strong { "PolymerVCS is currently in alpha." }
                    " Expect bugs, feature changes, and irrecoverable data loss."
                    br { }
                    strong { "Absolutely do not use for production workloads" }
                    ", for now."
                }
            }

            for platform in PLATFORMS {
                PlatformDownloads { key: "{platform.name}", platform }
            }
        }
    }
}
