#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod gcode;
mod cncmachine;

pub use app::TemplateApp;
pub use gcode::gcode_loader;
pub use cncmachine::cnc_machine;