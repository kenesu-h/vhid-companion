use serde::{Serialize, Deserialize};

use crate::{
    model::application_model::ApplicationModel,
    controller::stdio_interface::StdioInterface
};
use serde_json::json;

#[derive(Clone, Serialize, Deserialize)]
/// Commands accepted by this application.
///
/// This application accepts these commands through the controller, which reads
/// from stdin. As long as you correctly serialize these commands as JSON and
/// pass them through stdin, the controller should be able to parse them down
/// into their respective enumerations.
pub enum ApplicationCommand {
    GetAnarchyMode,
    SetAnarchyMode { anarchy_mode: bool },
    GetIp,
    SetIp { ip: String },
    GetDelay { i: usize },
    SetDelay { i: usize, delay: u8 },
    GetLeftDeadzone { i: usize },
    SetLeftDeadzone { i: usize, deadzone: f32 },
    GetRightDeadzone { i: usize },
    SetRightDeadzone { i: usize, deadzone: f32 },

    // RunScript { i: usize, script: Vec<SimpleEvent> },
    Connect,
    Disconnect,
    Exit,

    // These will probably go unused by the Tauri end, but will stay in case
    // anyone else finds a decent use for them.
    Swap { i: usize, j: usize },

    Unsupported
}

fn write(stdio_if: &mut StdioInterface, result: Result<String, String>) -> () {
    stdio_if.write(json!(result).to_string())
        .expect("Failed to write to stdout buffer.");
}

impl ApplicationCommand {
    pub fn execute(
        self, model: &mut ApplicationModel, stdio_if: &mut StdioInterface
    ) -> () {
        let ok: bool;
        let out: String;
        match self {
            Self::GetAnarchyMode => {
                match model.get_anarchy_mode() {
                    Err(e) => { ok = false; out = e },
                    Ok(anarchy_mode) => {
                        ok = true;
                        out = anarchy_mode.to_string()
                    }
                }
            },
            Self::SetAnarchyMode { anarchy_mode } => {
                match model.set_anarchy_mode(anarchy_mode) {
                    Err(e) => { ok = false; out = e },
                    Ok(_) => {
                        ok = true;
                        out = String::from("Successfully set anarchy mode.")
                    }
                }
            },
            Self::GetIp => match model.get_ip() {
                Err(e) => { ok = false; out = e },
                Ok(ip) => { ok = true; out = ip } 
            },
            Self::SetIp { ip } => match model.set_ip(ip) {
                Err(e) => { ok = false; out = e },
                Ok(_) => {
                    ok = true;
                    out = String::from("Successfully set IP.");
                }
            },
            Self::GetDelay { i } => {
                match model.get_delay(i) {
                    Err(e) => { ok = false; out = e },
                    Ok(delay) => { ok = true; out = delay.to_string() }
                }
            },
            Self::SetDelay { i, delay } => {
                match model.set_delay(i, delay) { 
                    Err(e) => { ok = false; out = e },
                    Ok(_) => {
                        ok = true;
                        out = format!(
                            "Successfully set delay of gamepad {}.", i)
                    }
                }
            },
            Self::GetLeftDeadzone { i } => {
                match model.get_left_deadzone(i) {
                    Err(e) => { ok = false; out = e },
                    Ok(deadzone) => {
                        ok = true;
                        out = deadzone.to_string()
                    }
                }
            },
            Self::SetLeftDeadzone {
                i, deadzone
            } => match model.set_left_deadzone(i, deadzone) {
                Err(e) => { ok = false; out = e },
                Ok(_) => {
                    ok = true;
                    out = format!(
                        "Successfully set left deadzone of gamepad {}.", i)
                }
            },
            Self::GetRightDeadzone { i } => {
                match model.get_right_deadzone(i) {
                    Err(e) => { ok = false; out = e },
                    Ok(deadzone) => {
                        ok = true;
                        out = deadzone.to_string()
                    }
                }
            },
            Self::SetRightDeadzone {
                i, deadzone
            } => match model.set_right_deadzone(i, deadzone) {
                Err(e) => { ok = false; out = e },
                Ok(_) => {
                    ok = true;
                    out = format!(
                        "Successfully set right deadzone of gamepad {}.", i)
                }
            },
            Self::Swap { i, j } => match model.swap(i, j) {
                Err(e) => { ok = false; out = e },
                Ok(_) => {
                    ok = true;
                    out = format!(
                        "Successfully swapped gamepads {} and {}.", i, j)
                }
            },
            Self::Connect => match model.connect() {
                Err(e) => { ok = false; out = e },
                Ok(_) => {
                    ok = true;
                    out = String::from("Now sending packets.")
                }
            },
            Self::Disconnect => match model.disconnect() {
                Err(e) => { ok = false; out = e },
                Ok(_) => {
                    ok = true;
                    out = String::from("No longer sending packets.")
                }
            },
            Self::Exit => match model.exit() {
                Err(e) => { ok = false; out = e },
                Ok(_) => {
                    ok = true;
                    out = String::from("Successfully exited.")
                }
            },
            Self::Unsupported => {
                ok = false;
                out = String::from("The given command is unsupported.")
            }
        }
        write(stdio_if, match ok {
            false => Err(out),
            true => Ok(out)
        });
    }
}
