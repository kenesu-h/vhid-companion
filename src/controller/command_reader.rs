use std::{
    time::Instant,
    sync::{
        Arc, Mutex,
        atomic::AtomicBool
    }
};

use crate::{
    model::application_model::ApplicationModel,
    controller::{
        application_command::ApplicationCommand,
        stdio_interface::StdioInterface
    }
};
use crossbeam_channel::Receiver;

pub struct CommandReader {
    model: ApplicationModel,
    stdio_if: StdioInterface
}

impl CommandReader {
    pub fn new(
        ticks: Receiver<Instant>, done: Arc<AtomicBool>,
        commands_mtx: Arc<Mutex<Vec<ApplicationCommand>>>
    ) -> CommandReader {
        let model_ticks: Receiver<Instant> = ticks.clone();
        let model_done: Arc<AtomicBool> = Arc::clone(&done);
        let model: ApplicationModel = ApplicationModel::new(
            model_ticks,
            model_done);

        let stdio_ticks: Receiver<Instant> = ticks.clone();
        let stdio_done: Arc<AtomicBool> = Arc::clone(&done);
        let stdio_mtx: Arc<Mutex<Vec<ApplicationCommand>>>
            = Arc::clone(&commands_mtx);

        return CommandReader {
            model: model,
            stdio_if: StdioInterface::new(stdio_ticks, stdio_done, stdio_mtx)
        }
    }

    pub fn update(
        &mut self, commands_mtx: &Arc<Mutex<Vec<ApplicationCommand>>>
    ) -> () {
        if let Ok(mut commands) = commands_mtx.lock() {
            while let Some(command) = commands.pop() {
                self.accept(command);
            }
        }
    }

    pub fn accept(&mut self, command: ApplicationCommand) -> () {
        command.execute(&mut self.model, &mut self.stdio_if);
    }
}
