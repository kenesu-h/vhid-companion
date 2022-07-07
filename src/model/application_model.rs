use std::{
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
    thread::self,
    time::Instant,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering}
    }, 
};

use crate::model::{
    input::{
        manager::GamepadManager,
        script_event::ScriptEvent
    },
    sdl_interface::{SdlReader, SdlWriter},
    sysmodule_interface::SysmoduleInterface
};
use crossbeam_channel::{select, Receiver};
use sdl_event_server::SdlEvent;

struct ApplicationState {
    connected: Arc<AtomicBool>,
    done: Arc<AtomicBool>
}

impl ApplicationState {
    pub fn new(done: Arc<AtomicBool>) -> ApplicationState {
        return ApplicationState {
            connected: Arc::new(AtomicBool::new(false)),
            done: done 
        }
    }

    pub fn get_connected(&self) -> Arc<AtomicBool> {
        return Arc::clone(&self.connected);
    }

    pub fn set_connected(&self, connected: bool) -> () {
        self.connected.store(connected, Ordering::SeqCst);
    }

    pub fn get_done(&self) -> Arc<AtomicBool> {
        return Arc::clone(&self.done);
    }

    pub fn set_done(&self, done: bool) -> () {
        self.done.store(done, Ordering::SeqCst);
    }
}

pub struct ApplicationModel {
    gamepad_manager_mtx: Arc<Mutex<GamepadManager>>,
    sys_if_mtx: Arc<Mutex<SysmoduleInterface>>,
    sdl_writer: SdlWriter,

    state: ApplicationState,
    update_thread: thread::JoinHandle<()>
}

impl ApplicationModel {
    pub fn new(
        ticks: Receiver<Instant>, done: Arc<AtomicBool>
    ) -> ApplicationModel {
        let mut events: Vec<SdlEvent> = vec!();
        let state_done: Arc<AtomicBool> = Arc::clone(&done);
        let state: ApplicationState = ApplicationState::new(state_done);

        let update_ticks: Receiver<Instant> = ticks.clone();
        let update_connected: Arc<AtomicBool> = state.get_connected();
        let update_done: Arc<AtomicBool> = Arc::clone(&done);

        let main_manager_mtx: Arc<Mutex<GamepadManager>>
            = Arc::new(Mutex::new(GamepadManager::new()));
        let update_manager_mtx: Arc<Mutex<GamepadManager>>
            = Arc::clone(&main_manager_mtx);

        let main_sys_if_mtx: Arc<Mutex<SysmoduleInterface>>
            = Arc::new(Mutex::new(SysmoduleInterface::new()));
        let update_sys_if_mtx: Arc<Mutex<SysmoduleInterface>>
            = Arc::clone(&main_sys_if_mtx);

        let mut server: Child = Command::new("./sdl_event_server")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute SDL server.");
        let server_stdout: ChildStdout = server.stdout
            .take()
            .expect("Failed to pipe server's stdout.");
        let server_stdin: ChildStdin = server.stdin
            .take()
            .expect("Failed to pipe server's stdin.");

        let mut update_reader: SdlReader = SdlReader::new(server_stdout);
        let main_writer: SdlWriter = SdlWriter::new(server, server_stdin);

        let update_thread: thread::JoinHandle<()> = thread::spawn(move || {
            while !update_done.load(Ordering::Relaxed) {
                select! {
                    recv(update_ticks) -> _ => {
                        if let Err(_) = update_reader.read_into(&mut events) {
                            update_done.store(true, Ordering::SeqCst);
                        }
                        if let Ok(mut manager) = update_manager_mtx.lock() {
                            manager.update(&mut events);
                            if update_connected.load(Ordering::Relaxed) {
                                if let Ok(sys_if) = update_sys_if_mtx.lock() {
                                    if let Err(_) = sys_if.udp_update(
                                        manager.get_anarchy_mode(),
                                        manager.get_gamepads()
                                    ) {
                                        update_done.store(true, Ordering::SeqCst);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        return ApplicationModel {
            gamepad_manager_mtx: main_manager_mtx,
            sys_if_mtx: main_sys_if_mtx,
            sdl_writer: main_writer,

            state: state,
            update_thread: update_thread
        }
    }

    pub fn get_anarchy_mode(&self) -> Result<bool, String> {
        if let Ok(gamepad_manager) = self.gamepad_manager_mtx.lock() {
            return Ok(gamepad_manager.get_anarchy_mode());
        } else {
            return Err(String::from("Failed to lock gamepad manager."));
        }
    }

    pub fn set_anarchy_mode(
        &mut self, anarchy_mode: bool
    ) -> Result<(), String> {
        if let Ok(mut gamepad_manager) = self.gamepad_manager_mtx.lock() {
            gamepad_manager.set_anarchy_mode(anarchy_mode);
            return Ok(());
        } else {
            return Err(String::from("Failed to lock gamepad manager."));
        }
    }

    pub fn get_done(&self) -> Arc<AtomicBool> {
        return self.state.get_done();
    }

    pub fn get_ips(&self) -> Result<Vec<String>, String> {
        if let Ok(sys_if) = self.sys_if_mtx.lock() {
            return Ok(sys_if.get_ips())
        } else {
            return Err(String::from("Failed to lock sysmodule interface."));
        }
    }

    pub fn set_ips(&mut self, ips: Vec<String>) -> Result<(), String> {
        if let Ok(mut sys_if) = self.sys_if_mtx.lock() {
            sys_if.set_ips(ips);
            return Ok(());
        } else {
            return Err(String::from("Failed to lock sysmodule interface."));
        }
    }

    pub fn get_delay(&self, i: usize) -> Result<usize, String> {
        if let Ok(gamepad_manager) = self.gamepad_manager_mtx.lock() {
            return Ok(gamepad_manager.get_delay(i));
        } else {
            return Err(String::from("Failed to lock gamepad manager."));
        }
    }

    pub fn set_delay(&mut self, i: usize, delay: usize) -> Result<(), String> {
        if let Ok(mut gamepad_manager) = self.gamepad_manager_mtx.lock() {
            gamepad_manager.set_delay(i, delay);
            return Ok(());
        } else {
            return Err(String::from("Failed to lock gamepad manager."));
        }
    }

    pub fn get_left_deadzone(&self, i: usize) -> Result<f32, String> {
        if let Ok(gamepad_manager) = self.gamepad_manager_mtx.lock() {
            return Ok(gamepad_manager.get_left_deadzone(i));
        } else {
            return Err(String::from("Failed to lock gamepad manager."));
        }
    }

    pub fn set_left_deadzone(
        &mut self, i: usize, deadzone: f32
    ) -> Result<(), String> {
        if let Ok(mut gamepad_manager) = self.gamepad_manager_mtx.lock() {
            gamepad_manager.set_left_deadzone(i, deadzone);
            return Ok(());
        } else {
            return Err(String::from("Failed to lock gamepad manager."));
        }
    }

    pub fn get_right_deadzone(&self, i: usize) -> Result<f32, String> {
        if let Ok(gamepad_manager) = self.gamepad_manager_mtx.lock() {
            return Ok(gamepad_manager.get_right_deadzone(i));
        } else {
            return Err(String::from("Failed to lock gamepad manager."));
        }
    }

    pub fn set_right_deadzone(
        &mut self, i: usize, deadzone: f32
    ) -> Result<(), String> {
        if let Ok(mut gamepad_manager) = self.gamepad_manager_mtx.lock() {
            gamepad_manager.set_right_deadzone(i, deadzone);
            return Ok(());
        } else {
            return Err(String::from("Failed to lock gamepad manager."));
        }
    }

    pub fn run_script(
        &mut self, i: usize, script: Vec<ScriptEvent>
    ) -> Result<(), String> {
        if self.state.get_connected().load(Ordering::Relaxed) {
            if let Ok(mut gamepad_manager) = self.gamepad_manager_mtx.lock() {
                gamepad_manager.run_script(i, script);
                return Ok(());
            } else {
                return Err(String::from("Failed to lock gamepad manager."));
            }
        } else {
            return Err(String::from("Cannot run script while disconnected."));
        }
    }

    pub fn swap(&mut self, i: usize, j: usize) -> Result<(), String> {
        if let Ok(mut gamepad_manager) = self.gamepad_manager_mtx.lock() {
            gamepad_manager.swap(i, j);
            return Ok(());
        } else {
            return Err(String::from("Failed to lock gamepad manager."));
        }
    }

    pub fn connect(&mut self) -> Result<(), String> {
        if let Ok(sys_if) = self.sys_if_mtx.lock() {
            if sys_if.get_ips().len() == 0 {
                return Err(String::from("Cannot connect without an IP."));
            } else {
                self.state.set_connected(true);
                return Ok(());
            }
        } else {
            return Err(String::from("Failed to lock sysmodule interface."));
        }
    }

    pub fn disconnect(&mut self) -> Result<(), String> {
        if !self.state.get_connected().load(Ordering::Relaxed) {
            self.state.set_connected(false);
            return Ok(());
        } else {
            return Err(String::from("Already disconnected from sysmodule."));
        }
    }

    pub fn exit(&mut self) -> Result<(), String> {
        self.state.set_connected(false);
        self.state.set_done(true);
        return self.sdl_writer.exit();
    }

    pub fn join(self) -> () {
        self.update_thread.join().expect("Failed to join update thread.");
    }
}
