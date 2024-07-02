// date_writer module

use std::fs::OpenOptions;
use std::io::prelude::*;

use super::TextDataWriter;


pub struct DataWriter {
    file: Option<std::fs::File>,
}


impl DataWriter {
    pub fn new() -> Self {
        Self { file: None }
    }
}


impl TextDataWriter for DataWriter{

    fn open(&mut self, p: &std::path::Path) -> Result<(), std::io::Error> {
        if self.file.is_some() {
            Err(std::io::Error::from(std::io::ErrorKind::AlreadyExists))
        } else {
            let f = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(p);

            if let Ok(f) = f {
                self.file = Some(f);
                Ok(())
            } else if let Err(e) = f {
                Err(e)
            } else {
                unreachable!();
            }
        }
    }

    fn close(&mut self) {
        self.file = None;
    }

    fn write(&mut self, line: &str) -> Result<(), std::io::Error> {
        if let Some(mut f) = self.file.as_ref() {
            f.write_all(line.as_bytes())
        } else {
            Err(std::io::Error::from(std::io::ErrorKind::NotConnected))
        }
    }
}
