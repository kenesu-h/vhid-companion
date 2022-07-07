mod model;
mod controller;

#[macro_use]
extern crate structure;

pub use model::input::script_event::ScriptEvent;
pub use controller::application_command::ApplicationCommand;
pub use controller::application_controller::ApplicationController;
