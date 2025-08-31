#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod gcode;

pub use gcode::gcode_loader;
pub use app::TemplateApp;
