use std::error::Error;

#[cfg(not(feature="gui"))]
mod tui;
#[cfg(not(feature="gui"))]
use crate::tui::tui;

#[cfg(feature="gui")]
mod gui;
#[cfg(feature="gui")]
use crate::gui::gui;

mod applogic;


fn main() {

    #[cfg(feature="gui")]
    gui();
    #[cfg(not(feature="gui"))]
    tui();

}

