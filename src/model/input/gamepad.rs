use sdl_event_server::{SdlAxis, SdlButton, SdlEvent};

const TRIGGER_ACTUATION: i16 = i16::MAX;

#[derive(Copy, Clone)]
pub enum GamepadType {
    Disconnected,
    SwitchProController,
    SidewaysLeftJoyCon,
    SidewaysRightJoyCon
}

pub enum GamepadButton {
    A, B, X, Y, LST, RST, L, R, ZL, ZR, Plus, Minus, DL, DU, DR, DD, LL, LU, LR,
    LD, RL, RU, RR, RD, SLL, SRL, SLR, SRR, Home, Capture
}

impl GamepadButton {
    pub fn to_bit(&self) -> i32 {
        match self {
            Self::A => return 1,
            Self::B => return 1 << 1,
            Self::X => return 1 << 2,
            Self::Y => return 1 << 3,
            Self::LST => return 1 << 4,
            Self::RST => return 1 << 5,
            Self::L => return 1 << 6,
            Self::R => return 1 << 7,
            Self::ZL => return 1 << 8,
            Self::ZR => return 1 << 9,
            Self::Plus => return 1 << 10,
            Self::Minus => return 1 << 11,
            Self::DL => return 1 << 12,
            Self::DU => return 1 << 13,
            Self::DR => return 1 << 14,
            Self::DD => return 1 << 15,
            Self::LL => return 1 << 16,
            Self::LU => return 1 << 17,
            Self::LR => return 1 << 18,
            Self::LD => return 1 << 19,
            Self::RL => return 1 << 20,
            Self::RU => return 1 << 21,
            Self::RR => return 1 << 22,
            Self::RD => return 1 << 23,
            Self::SLL => return 1 << 24,
            Self::SRL => return 1 << 25,
            Self::SLR => return 1 << 26,
            Self::SRR => return 1 << 27,
            Self::Home => return 1 << 18,
            Self::Capture => return 1 << 19
        }
    }

    pub fn from_sdl(
        button: SdlButton, gamepad_type: &GamepadType
    ) -> Result<GamepadButton, String> {
        match button {
            // TODO: Map gamepad types later.
            SdlButton::A => Ok(Self::B),
            SdlButton::B => Ok(Self::A),
            SdlButton::X => Ok(Self::Y),
            SdlButton::Y => Ok(Self::X),
            SdlButton::Back => Ok(Self::Minus),
            SdlButton::Misc1 => Ok(Self::Capture),
            SdlButton::Guide => Ok(Self::Home),
            SdlButton::Start => Ok(Self::Plus),
            SdlButton::LeftStick => Ok(Self::LST),
            SdlButton::RightStick => Ok(Self::RST),
            SdlButton::LeftShoulder => Ok(Self::L),
            SdlButton::RightShoulder => Ok(Self::R),
            SdlButton::DPadUp => Ok(Self::DU),
            SdlButton::DPadDown => Ok(Self::DD),
            SdlButton::DPadLeft => Ok(Self::DL),
            SdlButton::DPadRight => Ok(Self::DR),
            _ => Err(String::from("Button is unmapped."))
        }
    }
}

#[derive(Copy, Clone)]
pub struct AnalogStick {
    position: (i16, i16),
    deadzone: f32
}

impl AnalogStick {
    pub fn new() -> AnalogStick {
        return AnalogStick {
            position: (0, 0),
            deadzone: 0.0
        }
    }

    pub fn get_position(&self) -> (i16, i16) {
        // I tried my best to make this look somewhat reasonable.
        if f32::sqrt(
            ((self.position.0.pow(2) as i32)
             + (self.position.1.pow(2) as i32)) as f32)
            <= self.deadzone * (i16::MAX as f32) {
            return (0, 0);
        } else {
            return self.position;
        }
    }

    pub fn get_deadzone(&self) -> f32 {
        return self.deadzone;
    }

    pub fn set_deadzone(&mut self, deadzone: f32) -> () {
        self.deadzone = deadzone;
    }

    pub fn update(&mut self, axis: SdlAxis, value: i16) -> Result<(), String> {
        match axis {
            SdlAxis::LeftX | SdlAxis::RightX => {
                self.position.0 = value;
                return Ok(());
            },
            SdlAxis::LeftY | SdlAxis::RightY => {
                self.position.1 = value;
                return Ok(());
            },
            _ => Err(String::from(
                    "AnalogStick.update() doesn't support triggers."))
        }
    }

    pub fn reset(&mut self) -> () {
        self.position = (0, 0);
        self.deadzone = 0.0;
    }

    pub fn merge(&mut self, other: AnalogStick) -> () {
        self.position = (
            i16::saturating_add(self.position.0, other.get_position().0),
            i16::saturating_add(self.position.1, other.get_position().1)
        );
    }
}

#[derive(Copy, Clone)]
pub struct Gamepad {
    gamepad_type: GamepadType,
    delay: usize,

    buttons: i32,
    left_stick: AnalogStick,
    right_stick: AnalogStick 
}

impl Gamepad {
    pub fn new() -> Gamepad {
        return Gamepad {
            gamepad_type: GamepadType::Disconnected,
            delay: 0,

            buttons: 0,
            left_stick: AnalogStick::new(),
            right_stick: AnalogStick::new()
        }
    }

    pub fn get_gamepad_type(&self) -> &GamepadType {
        return &self.gamepad_type;
    }

    pub fn get_delay(&self) -> usize {
        return self.delay;
    }

    pub fn set_delay(&mut self, delay: usize) -> () {
        self.delay = delay;
    }

    pub fn get_buttons(&self) -> i32 {
        return self.buttons;
    }

    pub fn get_left_stick(&self) -> AnalogStick {
        return self.left_stick.clone();
    }

    pub fn get_left_deadzone(&self) -> f32 {
        return self.left_stick.get_deadzone();
    }

    pub fn set_left_deadzone(&mut self, deadzone: f32) -> () {
        self.left_stick.set_deadzone(deadzone);
    }

    pub fn get_right_stick(&self) -> AnalogStick {
        return self.right_stick.clone();
    }

    pub fn get_right_deadzone(&self) -> f32 {
        return self.right_stick.get_deadzone();
    }

    pub fn set_right_deadzone(&mut self, deadzone: f32) -> () {
        self.right_stick.set_deadzone(deadzone);
    }

    pub fn connect(&mut self, gamepad_type: GamepadType) -> () {
        self.gamepad_type = gamepad_type;
    }

    pub fn disconnect(&mut self) -> () {
        self.gamepad_type = GamepadType::Disconnected;
        self.delay = 0;

        self.buttons = 0;
        self.left_stick.reset();
        self.right_stick.reset();
    }

    pub fn update(&mut self, event: SdlEvent) -> Result<(), String> {
        match event {
            SdlEvent::AxisMotion { timestamp: _, which: _, axis, value }
                => if !axis.is_trigger() {
                    self.update_axis(axis, value).unwrap();
                    return Ok(());
                } else {
                    self.update_trigger(axis, value).unwrap();
                    return Ok(());
                },
            SdlEvent::ButtonPress { timestamp: _, which: _, button, pressed }
                => match GamepadButton::from_sdl(button, &self.gamepad_type) {
                    Err(e) => Err(e),
                    Ok(mapped) => {
                        self.update_button(mapped, pressed);
                        return Ok(());
                    }
                },
            _ => Err(String::from(
                    "Gamepad.update() doesn't support controller addition and \
                    removal events."))
        }
    }

    fn update_axis(&mut self, axis: SdlAxis, value: i16) -> Result<(), String> {
        match axis {
            SdlAxis::LeftX | SdlAxis::LeftY => {
                self.left_stick.update(axis, value).unwrap();
                return Ok(());
            },
            SdlAxis::RightX | SdlAxis::RightY => {
                self.right_stick.update(axis, value).unwrap();
                return Ok(());
            },
            _ => Err(String::from(
                    "Gamepad.update_axis() doesn't support triggers."))
        }
    }

    fn update_trigger(
        &mut self, trigger: SdlAxis, value: i16
    ) -> Result<(), String> {
        match trigger {
            SdlAxis::TriggerLeft => {
                self.update_button(GamepadButton::ZL, value > TRIGGER_ACTUATION);
                return Ok(());
            },
            SdlAxis::TriggerRight => {
                self.update_button(GamepadButton::ZR, value > TRIGGER_ACTUATION);
                return Ok(());
            },
            _ => Err(String::from(
                    "Gamepad.update_trigger() doesn't support axes."))
        }
    }

    /// Updates one of this virtual gamepad's buttons using bitwise operations.
    ///
    /// Mostly derived from Pask's original code, but this also just happens to
    /// be a really nice way of toggling specific button bits.
    fn update_button(&mut self, button: GamepadButton, pressed: bool) -> () {
        if pressed {
            // Toggles button bit on with a bitwise OR.
            self.buttons |= button.to_bit();
        } else {
            // Toggles button bit off with a bitwise AND.
            self.buttons &= !button.to_bit();
        }
    }

    pub fn merge(&mut self, other: Gamepad) -> () {
        self.buttons |= other.get_buttons();
        self.left_stick.merge(other.get_left_stick());
        self.right_stick.merge(other.get_right_stick());
    }
}
