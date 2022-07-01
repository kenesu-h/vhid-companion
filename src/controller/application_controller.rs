use std::{
    thread,
    time::{self, Instant},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering}
    }
};

use crate::controller::{
    application_command::ApplicationCommand,
    command_reader::CommandReader
};
use crossbeam_channel::{tick, select, Receiver};

const FREQUENCY: i32 = 60;

pub struct ApplicationController {
    command_thread: thread::JoinHandle<()>
}

impl ApplicationController {
    pub fn new() -> ApplicationController {
        let ticks: Receiver<Instant> = tick(
            time::Duration::from_secs_f32(1.0 / FREQUENCY as f32));
        let done: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

        let commands_ticks: Receiver<Instant> = ticks.clone();
        let commands_done: Arc<AtomicBool> = Arc::clone(&done);
        let commands_mtx: Arc<Mutex<Vec<ApplicationCommand>>>
            = Arc::new(Mutex::new(vec!()));

        let thread_ticks: Receiver<Instant> = ticks.clone();
        let thread_done: Arc<AtomicBool> = Arc::clone(&done);
        let thread_mtx: Arc<Mutex<Vec<ApplicationCommand>>>
            = Arc::clone(&commands_mtx);

        let mut command_reader: CommandReader = CommandReader::new(
            commands_ticks,
            commands_done,
            commands_mtx
        );

        let command_thread: thread::JoinHandle<()> = thread::spawn(move || {
            while !thread_done.load(Ordering::Relaxed) {
                select! {
                    recv(thread_ticks) -> _ => {
                        command_reader.update(&thread_mtx);
                    }
                }
            }
        });

        return ApplicationController {
            command_thread: command_thread
        }
    }

    pub fn join(self) -> () {
        self.command_thread.join().expect("Failed to join command thread.");
    }
}
