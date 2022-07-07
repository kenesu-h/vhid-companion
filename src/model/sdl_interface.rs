use std::{
    io::{BufReader, BufRead, BufWriter, Write},
    process::{Child, ChildStdin, ChildStdout},
};

use sdl_event_server::SdlEvent;
use serde_json;

pub struct SdlReader {
    reader: BufReader<ChildStdout>
}

impl SdlReader {
    pub fn new(stdout: ChildStdout) -> SdlReader {
        return SdlReader {
            reader: BufReader::new(stdout)
        }
    }

    pub fn has_data(&mut self) -> bool {
        match self.reader.fill_buf() {
            Err(_) => false,
            Ok(bytes) => bytes.len() != 0
        }
    }

    pub fn read_into(
        &mut self, events: &mut Vec<SdlEvent>
    ) -> Result<(), String> {
        let mut buffer: String = String::new();
        while self.has_data() {
            if let Err(_) = self.reader.read_line(&mut buffer) {
                return Err(String::from("Failed to read from server buffer."));
            }
            let maybe_event: serde_json::Result<SdlEvent>
                = serde_json::from_str(buffer.trim());
            match maybe_event {
                Err(_) => (),
                // TODO: Double check that this pushes in the right order
                Ok(event) => events.push(event)
            }
        }
        return Ok(());
    }
}

pub struct SdlWriter {
    server: Child,
    writer: BufWriter<ChildStdin>
}

impl SdlWriter {
    pub fn new(server: Child, stdin: ChildStdin) -> SdlWriter {
        return SdlWriter {
            server: server,
            writer: BufWriter::new(stdin)
        }
    }

    pub fn exit(&mut self) -> Result<(), String> {
        match self.writer.write("exit\n".as_bytes()) {
            Err(_) => match self.server.kill() {
                Err(_) => Err(String::from("Failed to write to server buffer and kill it.")),
                Ok(_) => Err(String::from("Failed to write to server buffer."))
            },
            Ok(_) => match self.writer.flush() {
                Err(_) => Err(String::from("Failed to flush server buffer.")),
                Ok(_) => Ok(())
            }
        }
    }
}
