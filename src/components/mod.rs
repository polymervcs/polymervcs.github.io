//! Components shared between pages.

mod copy_button;
mod icon_svg;
mod nav_link;
mod platform_downloads;
mod theme_button;

pub use copy_button::CopyButton;
pub use icon_svg::IconSvg;
// Only the header in `Shell` uses it, and that's commented out.
#[allow(unused_imports)]
pub use nav_link::NavLink;
pub use platform_downloads::{Download, Package, Platform, PlatformDownloads};
pub use theme_button::{THEME_SCRIPT, ThemeButton};
