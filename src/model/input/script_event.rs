use sdl_event_server::{SdlAxis, SdlButton};
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum ScriptEvent {
    AxisMotion { axis: SdlAxis, value: i16 },
    ButtonPress { button: SdlButton, pressed: bool },
    Wait { frames: usize }
}
