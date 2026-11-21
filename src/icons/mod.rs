mod logos;
mod ui;

pub use logos::*;
pub use ui::*;

#[derive(PartialEq)]
pub struct Icon {
    pub view_box: &'static str,
    pub path: &'static str,
}
