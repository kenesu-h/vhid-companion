use std::collections::HashMap;

use crate::model::input::{
    gamepad::{GamepadType, Gamepad},
    script_event::ScriptEvent
};
use sdl_event_server::{SdlButton, SdlEvent};

pub const NUM_GAMEPADS: usize = 8;
const CONNECT_BUTTON: SdlButton = SdlButton::RightShoulder;

pub struct GamepadManager {
    anarchy_mode: bool,
    gamepads: [Gamepad; NUM_GAMEPADS],
    indices: HashMap<usize, usize>,

    buffer: Vec<(SdlEvent, usize)>
}

impl GamepadManager {
    pub fn new() -> GamepadManager {
        return GamepadManager {
            anarchy_mode: false,
            gamepads: [Gamepad::new(); NUM_GAMEPADS],
            indices: HashMap::new(),

            buffer: vec!()
        }
    }

    pub fn get_anarchy_mode(&self) -> bool {
        return self.anarchy_mode;
    }

    pub fn set_anarchy_mode(&mut self, anarchy_mode: bool) -> () {
        self.anarchy_mode = anarchy_mode;
    }

    pub fn get_gamepads(&self) -> [Gamepad; NUM_GAMEPADS] {
        return self.gamepads.clone();
    }

    pub fn get_delay(&self, i: usize) -> usize {
        return self.gamepads[i].get_delay();
    }

    pub fn set_delay(&mut self, i: usize, delay: usize) -> () {
        self.gamepads[i].set_delay(delay);
    }

    pub fn get_left_deadzone(&self, i: usize) -> f32 {
        return self.gamepads[i].get_left_deadzone();
    }

    pub fn set_left_deadzone(&mut self, i: usize, deadzone: f32) -> () {
        self.gamepads[i].set_left_deadzone(deadzone);
    }

    pub fn get_right_deadzone(&self, i: usize) -> f32 {
        return self.gamepads[i].get_right_deadzone();
    }

    pub fn set_right_deadzone(&mut self, i: usize, deadzone: f32) -> () {
        self.gamepads[i].set_right_deadzone(deadzone);
    }

    pub fn read_script_event(&mut self, i: usize, event: ScriptEvent) -> () {
        let delay: usize = 0;
    }

    pub fn swap(&mut self, i: usize, j: usize) -> () {
        let temp: Gamepad = self.gamepads[i];
        self.gamepads[i] = self.gamepads[j];
        self.gamepads[j] = temp;

        // Remember to also change self.indices ("which" -> indices).
        let mut temp_i: usize = 0;
        let mut temp_j: usize = 0;
        for key in self.indices.keys() {
            if self.indices.get(key).unwrap() == &i {
                temp_i = *key;
            } else if self.indices.get(key).unwrap() == &j {
                temp_j = *key;
            }
        }
        self.indices.insert(temp_i, j);
        self.indices.insert(temp_j, i);
    }

    pub fn update(&mut self, events: &mut Vec<SdlEvent>) -> () {
        self.read_events(events);
        self.read_buffer();
    }

    fn read_events(&mut self, events: &mut Vec<SdlEvent>) -> () {
        while let Some(event) = events.pop() {
            let i: u32;
            match event {
                SdlEvent::ControllerAdded {
                    timestamp: _,
                    which
                } => {
                    self.connect(which);
                    continue;
                },
                SdlEvent::ControllerRemoved {
                    timestamp: _,
                    which
                } => {
                    self.disconnect(which);
                    continue;
                },
                SdlEvent::AxisMotion {
                    timestamp: _,
                    which,
                    axis: _,
                    value: _
                } => i = which,
                SdlEvent::ButtonPress {
                    timestamp: _,
                    which,
                    button: _,
                    pressed: _
                } => i = which
            }
            if let Some(j) = self.indices.get(&(i as usize)) {
                self.buffer.insert(
                    0,
                    (event, self.gamepads[*j].get_delay())
                );
            } else {
                self.buffer.insert(0, (event, 0));
            }
        }
    }

    fn read_buffer(&mut self) -> () {
        // We're only going to loop the original buffer length.
        let mut i = 0;
        let mut buffer_len: usize = self.buffer.len();

        while i < buffer_len {
            if let Some((event, delay)) = self.buffer.pop() {
                if delay == 0 {
                    if let Some(i) = self.get_indices(event) {
                        if let Err(e) = self.gamepads[*i].update(event) {
                            eprintln!("{}", e);
                        }
                    } else {
                        if let SdlEvent::ButtonPress {
                            timestamp: _,
                            which,
                            button: CONNECT_BUTTON,
                            pressed: true
                        } = event {
                            self.connect(which);
                        }
                    }
                } else {
                    self.buffer.insert(0, (event, delay - 1));
                    i += 1;
                    buffer_len += 1;
                }
            }
        }
    }

    fn get_indices(&self, event: SdlEvent) -> Option<&usize> {
        let i: u32;
        match event {
            SdlEvent::ControllerAdded { timestamp: _, which }
                => i = which,
            SdlEvent::ControllerRemoved { timestamp: _, which }
                => i = which,
            SdlEvent::AxisMotion { timestamp: _, which, axis: _, value: _ }
                => i = which,
            SdlEvent::ButtonPress { timestamp: _, which, button: _, pressed: _ }
                => i = which
        }
        return self.indices.get(&(i as usize));
    }

    fn connect(&mut self, which: u32) -> () {
        for i in 0..NUM_GAMEPADS {
            if let None = self.gamepads.get(which as usize) {
                self.gamepads[i].connect(GamepadType::SwitchProController);
                self.indices.insert(which as usize, i); 
            }
        }
    }

    fn disconnect(&mut self, which: u32) -> () {
        self.gamepads[*self.indices.get(&(which as usize)).unwrap()]
            .disconnect();
        self.indices.remove(&(which as usize));
    }
}
