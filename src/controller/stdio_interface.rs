use std::{
    io,
    io::{BufReader, BufRead, BufWriter, Write},
    thread,
    time::Instant,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering}
    }
};

use crate::controller::application_command::ApplicationCommand;
use crossbeam_channel::{select, Receiver};
use serde_json;

struct StdinReader {
    reader: BufReader<io::Stdin>,
}

impl StdinReader {
    pub fn new() -> StdinReader {
        return StdinReader { reader: BufReader::new(io::stdin()) }
    }

    pub fn has_data(&mut self) -> bool {
        match self.reader.fill_buf() {
            Err(_) => false,
            Ok(bytes) => bytes.len() != 0
        }
    }

    pub fn read_into(
        &mut self, commands_mtx: Arc<Mutex<Vec<ApplicationCommand>>>
    ) -> Result<(), String> {
        let mut buffer: String = String::new();
        while self.has_data() {
            if let Err(_) = self.reader.read_line(&mut buffer) {
                return Err(String::from("Failed to read from stdin buffer."));
            }
            let maybe_command: serde_json::Result<ApplicationCommand>
                = serde_json::from_str(buffer.trim());
            if let Ok(mut commands) = commands_mtx.lock() {
                match maybe_command {
                    Err(_) => commands.push(ApplicationCommand::Unsupported),
                    Ok(command) => commands.push(command)
                }
            } else {
                return Err(String::from("Failed to lock commands."));
            }
            buffer.clear();
        }
        return Ok(());
    }
}

struct StdoutWriter {
    writer: BufWriter<io::Stdout>
}

impl StdoutWriter {
    pub fn new() -> StdoutWriter {
        return StdoutWriter {
            writer: BufWriter::new(io::stdout())
        }
    }

    pub fn write(&mut self, s: String) -> Result<(), String> {
        let mut clone: String = s.clone();
        clone.push('\n');
        let bytes: &[u8] = clone.as_bytes();
        if let Err(_) = self.writer.write(bytes) {
            return Err(String::from("Failed to write to stdout buffer."));
        } else {
            if let Err(_) = self.writer.flush() {
                return Err(String::from("Failed to flush stdout buffer"));
            }
        }
        return Ok(());
    }
}

pub struct StdioInterface {
    read_thread: thread::JoinHandle<()>,
    writer: StdoutWriter
}

impl StdioInterface {
    pub fn new(
        ticks: Receiver<Instant>, done: Arc<AtomicBool>,
        commands_mtx: Arc<Mutex<Vec<ApplicationCommand>>>
    ) -> StdioInterface {
        let mut reader: StdinReader = StdinReader::new();
        let read_done: Arc<AtomicBool> = Arc::clone(&done);
        let read_ticks: Receiver<Instant> = ticks.clone(); 
        let read_mtx: Arc<Mutex<Vec<ApplicationCommand>>>
            = Arc::clone(&commands_mtx);

        let read_thread: thread::JoinHandle<()> = thread::spawn(move || {
            while !read_done.load(Ordering::Relaxed) {
                select! {
                    recv(read_ticks) -> _ => {
                        match reader.read_into(read_mtx.clone()) {
                            Err(_) => read_done.store(true, Ordering::SeqCst),
                            Ok(_) => ()
                        }
                    }
                }
            }
        });

        return StdioInterface {
            read_thread: read_thread,
            writer: StdoutWriter::new()
        }
    }

    pub fn write(&mut self, s: String) -> Result<(), String> {
        return self.writer.write(s);
    }

    pub fn join(self) -> () {
        self.read_thread.join().expect("Failed to join read thread.");
    }
}
