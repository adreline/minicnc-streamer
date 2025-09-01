#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod cncmachine;
mod gcode;

pub use app::TemplateApp;
pub use cncmachine::cnc_machine;
pub use gcode::gcode_loader;
