use std::{
    io::{BufReader, BufRead, BufWriter, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
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

pub struct SdlInterface {
    reader: SdlReader,
    writer: SdlWriter
}

impl SdlInterface {
    // https://stackoverflow.com/a/49597789
    pub fn new(
    ) -> SdlInterface {
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

        return SdlInterface {
            reader: SdlReader::new(server_stdout),
            writer: SdlWriter::new(server, server_stdin)
        }
    }

    pub fn read_into(
        &mut self, events: &mut Vec<SdlEvent>
    ) -> Result<(), String> {
        return self.reader.read_into(events);
    }

    pub fn exit(&mut self) -> Result<(), String> { 
        return self.writer.exit();
    }
}
