use duckbill::duckfile::duckerror::DuckError;

mod tui;
mod applogic;

use crate::tui::tui;


fn main() -> Result<(), DuckError> {
    tui()
}

