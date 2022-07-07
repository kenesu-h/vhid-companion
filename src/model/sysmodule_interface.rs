use std::{
    net::UdpSocket
};

use crate::model::input::{
    gamepad::{GamepadType, Gamepad},
    manager::NUM_GAMEPADS
};

fn gamepad_type_to_u16(gamepad_type: &GamepadType) -> u16 {
    match gamepad_type {
        GamepadType::Disconnected => 0,
        GamepadType::SwitchProController => 1,
        GamepadType::SidewaysLeftJoyCon => 2,
        GamepadType::SidewaysRightJoyCon => 3
    }
}

struct UdpPacket {
    magic: u16, connected: u16,

    con_type: u16, keys: u64,
    joy_l_x: i32, joy_l_y: i32, joy_r_x: i32, joy_r_y: i32,

    con_type2: u16, keys2: u64,
    joy_l_x2: i32, joy_l_y2: i32, joy_r_x2: i32, joy_r_y2: i32,

    con_type3: u16, keys3: u64,
    joy_l_x3: i32, joy_l_y3: i32, joy_r_x3: i32, joy_r_y3: i32,

    con_type4: u16, keys4: u64,
    joy_l_x4: i32, joy_l_y4: i32, joy_r_x4: i32, joy_r_y4: i32,

    con_type5: u16, keys5: u64,
    joy_l_x5: i32, joy_l_y5: i32, joy_r_x5: i32, joy_r_y5: i32,

    con_type6: u16, keys6: u64,
    joy_l_x6: i32, joy_l_y6: i32, joy_r_x6: i32, joy_r_y6: i32,

    con_type7: u16, keys7: u64,
    joy_l_x7: i32, joy_l_y7: i32, joy_r_x7: i32, joy_r_y7: i32,

    con_type8: u16, keys8: u64,
    joy_l_x8: i32, joy_l_y8: i32, joy_r_x8: i32, joy_r_y8: i32
}

impl UdpPacket {
    pub fn new(gamepads: [Gamepad; NUM_GAMEPADS]) -> UdpPacket {
        return UdpPacket {
            magic: 0x3276, connected: 8,

            con_type: gamepad_type_to_u16(gamepads[0].get_gamepad_type()),
            keys: gamepads[0].get_buttons() as u64,
            joy_l_x: gamepads[0].get_left_stick().get_position().0 as i32,
            joy_l_y: gamepads[0].get_left_stick().get_position().1 as i32,
            joy_r_x: gamepads[0].get_right_stick().get_position().0 as i32,
            joy_r_y: gamepads[0].get_right_stick().get_position().1 as i32,

            con_type2: gamepad_type_to_u16(gamepads[1].get_gamepad_type()),
            keys2: gamepads[1].get_buttons() as u64,
            joy_l_x2: gamepads[1].get_left_stick().get_position().0 as i32,
            joy_l_y2: gamepads[1].get_left_stick().get_position().1 as i32,
            joy_r_x2: gamepads[1].get_right_stick().get_position().0 as i32,
            joy_r_y2: gamepads[1].get_right_stick().get_position().1 as i32,

            con_type3: gamepad_type_to_u16(gamepads[2].get_gamepad_type()),
            keys3: gamepads[2].get_buttons() as u64,
            joy_l_x3: gamepads[2].get_left_stick().get_position().0 as i32,
            joy_l_y3: gamepads[2].get_left_stick().get_position().1 as i32,
            joy_r_x3: gamepads[2].get_right_stick().get_position().0 as i32,
            joy_r_y3: gamepads[2].get_right_stick().get_position().1 as i32,

            con_type4: gamepad_type_to_u16(gamepads[3].get_gamepad_type()),
            keys4: gamepads[3].get_buttons() as u64,
            joy_l_x4: gamepads[3].get_left_stick().get_position().0 as i32,
            joy_l_y4: gamepads[3].get_left_stick().get_position().1 as i32,
            joy_r_x4: gamepads[3].get_right_stick().get_position().0 as i32,
            joy_r_y4: gamepads[3].get_right_stick().get_position().1 as i32,

            con_type5: gamepad_type_to_u16(gamepads[4].get_gamepad_type()),
            keys5: gamepads[4].get_buttons() as u64,
            joy_l_x5: gamepads[4].get_left_stick().get_position().0 as i32,
            joy_l_y5: gamepads[4].get_left_stick().get_position().1 as i32,
            joy_r_x5: gamepads[4].get_right_stick().get_position().0 as i32,
            joy_r_y5: gamepads[4].get_right_stick().get_position().1 as i32,

            con_type6: gamepad_type_to_u16(gamepads[5].get_gamepad_type()),
            keys6: gamepads[5].get_buttons() as u64,
            joy_l_x6: gamepads[5].get_left_stick().get_position().0 as i32,
            joy_l_y6: gamepads[5].get_left_stick().get_position().1 as i32,
            joy_r_x6: gamepads[5].get_right_stick().get_position().0 as i32,
            joy_r_y6: gamepads[5].get_right_stick().get_position().1 as i32,

            con_type7: gamepad_type_to_u16(gamepads[6].get_gamepad_type()),
            keys7: gamepads[6].get_buttons() as u64,
            joy_l_x7: gamepads[6].get_left_stick().get_position().0 as i32,
            joy_l_y7: gamepads[6].get_left_stick().get_position().1 as i32,
            joy_r_x7: gamepads[6].get_right_stick().get_position().0 as i32,
            joy_r_y7: gamepads[6].get_right_stick().get_position().1 as i32,

            con_type8: gamepad_type_to_u16(gamepads[7].get_gamepad_type()),
            keys8: gamepads[7].get_buttons() as u64,
            joy_l_x8: gamepads[7].get_left_stick().get_position().0 as i32,
            joy_l_y8: gamepads[7].get_left_stick().get_position().1 as i32,
            joy_r_x8: gamepads[7].get_right_stick().get_position().0 as i32,
            joy_r_y8: gamepads[7].get_right_stick().get_position().1 as i32
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        // H - controller type
        // Q - keyout
        // i - stick info
        structure!("<HHHQiiiiHQiiiiHQiiiiHQiiiiHQiiiiHQiiiiHQiiiiHQiiii").pack(
          self.magic, self.connected,

          self.con_type, self.keys,
          self.joy_l_x, self.joy_l_y, self.joy_r_x, self.joy_r_y,

          self.con_type2, self.keys2,
          self.joy_l_x2, self.joy_l_y2, self.joy_r_x2, self.joy_r_y2,

          self.con_type3, self.keys3,
          self.joy_l_x3, self.joy_l_y3, self.joy_r_x3, self.joy_r_y3,

          self.con_type4, self.keys4,
          self.joy_l_x4, self.joy_l_y4, self.joy_r_x4, self.joy_r_y4,

          self.con_type5, self.keys5,
          self.joy_l_x5, self.joy_l_y5, self.joy_r_x5, self.joy_r_y5,

          self.con_type6, self.keys6,
          self.joy_l_x6, self.joy_l_y6, self.joy_r_x6, self.joy_r_y6,

          self.con_type7, self.keys7,
          self.joy_l_x7, self.joy_l_y7, self.joy_r_x7, self.joy_r_y7,

          self.con_type8, self.keys8,
          self.joy_l_x8, self.joy_l_y8, self.joy_r_x8, self.joy_r_y8
        ).unwrap()
    }
}

struct SysmoduleUdpWriter {
    ips: Vec<String>,
    formatted: Vec<String>,
    writer: UdpSocket
}

impl SysmoduleUdpWriter {
    pub fn new(udp: UdpSocket) -> SysmoduleUdpWriter {
        return SysmoduleUdpWriter {
            ips: vec!(),
            formatted: vec!(),
            writer: udp
        }
    }

    pub fn get_ips(&self) -> Vec<String> {
        return self.ips.clone();
    }

    pub fn set_ips(&mut self, ips: Vec<String>) -> () {
        self.ips = ips;
        let mut formatted: Vec<String> = vec!();
        for ip in &self.ips {
            formatted.push(format!("{}:8000", ip));
        }
        self.formatted = formatted;
    }

    pub fn write(
        &self, anarchy_mode: bool, gamepads: [Gamepad; NUM_GAMEPADS]
    ) -> Result<(), String> {
        let packet: UdpPacket = self.create_packet(anarchy_mode, gamepads);
        let bytes: Vec<u8> = packet.as_bytes();
        for ip in &self.formatted {
            if let Err(_) = self.writer.send_to(&bytes, ip) {
                return Err(String::from("Failed to send packet to sysmodule."));
            }
        }
        return Ok(());
    }

    fn create_packet(
        &self, anarchy_mode: bool, gamepads: [Gamepad; NUM_GAMEPADS]
    ) -> UdpPacket {
        // I forget why, but I apparently also checked in the original code if
        // there was at least one controller connected.
        if anarchy_mode {
            let mut anarchy_gamepads: [Gamepad; NUM_GAMEPADS]
                = [Gamepad::new(); NUM_GAMEPADS];
            for gamepad in gamepads {
                anarchy_gamepads[0].merge(gamepad);
            }
            return UdpPacket::new(anarchy_gamepads);
        } else {
            return UdpPacket::new(gamepads);
        }
    }
}

pub struct SysmoduleInterface {
    udp_writer: SysmoduleUdpWriter
}

impl SysmoduleInterface {

    pub fn new() -> SysmoduleInterface {
        let udp: UdpSocket = UdpSocket::bind("0.0.0.0:8000")
            .expect("Failed to bind UDP socket.");
        
        return SysmoduleInterface {
            udp_writer: SysmoduleUdpWriter::new(udp)
        }
    }

    pub fn get_ips(&self) -> Vec<String> {
        return self.udp_writer.get_ips();
    }

    pub fn set_ips(&mut self, ips: Vec<String>) -> () {
        self.udp_writer.set_ips(ips);
    }

    pub fn udp_update(
        &self, anarchy_mode: bool, gamepads: [Gamepad; NUM_GAMEPADS]
    ) -> Result<(), String> {
        return self.udp_writer.write(anarchy_mode, gamepads);
    }
}
