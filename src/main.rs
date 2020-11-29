//==============================================================================
// Open crates and libraries
//==============================================================================
extern crate iced;
extern crate image;
extern crate rayon;
extern crate num;

mod ui;
mod fractal;

use iced::Sandbox;
use iced::Settings;
use ui::app::MainWindow as app;

//==============================================================================
// Run
//==============================================================================

pub fn main() -> iced::Result {
    app::run(Settings::default())
}
