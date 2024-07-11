// Mock Data Writer module

#![cfg(test)]

use super::data_writer::TextDataWriter;
use std::sync::mpsc;


pub fn get_mock_text_data_writer() -> (MockTextFile, MockDataWriter){
  let (tx,rx) = mpsc::channel();
  let f = MockTextFile::new(rx);
  let w = MockDataWriter::new(tx);
  ( f, w )
}


pub struct MockTextFile{
    rx: mpsc::Receiver::<String>,
    mock_data: Vec::<String>,
}


impl MockTextFile {
    fn new(rx: mpsc::Receiver::<String>) -> Self {
        Self {
            mock_data: vec!(),
            rx: rx,
        }
    }

    pub fn process_queued_messages(&mut self){
        let mut it = self.rx.try_iter();
        loop {
            if let Some(s) = it.next(){
                self.mock_data.push(s);
            }
            else{
                break;
            }
        }
    }

    pub fn get_mock_data(&self) -> &Vec::<String>{
      return &self.mock_data;
    }
}


pub struct MockDataWriter {
    file: Option<std::path::PathBuf>,
    tx: mpsc::Sender::<String>,
}


impl MockDataWriter {
    fn new(tx: mpsc::Sender::<String>) -> Self {
        Self {
            file: None, 
            tx: tx,
        }
    }
}


impl TextDataWriter for MockDataWriter{
    fn open(&mut self, p: &std::path::Path) -> Result<(), std::io::Error> {
        if self.file.is_some() {
            Err(std::io::Error::from(std::io::ErrorKind::AlreadyExists))
        }
        else {
          self.file = Some( p.to_path_buf() );
          Ok(())
        }
    }

    fn close(&mut self) {
        self.file = None;
        // Do not clear mock data here as it will normally be checked after file is closed
    }

    fn write(&mut self, line: &str) -> Result<(), std::io::Error> {
        if self.file.is_some() {
            assert!(self.tx.send( String::from(line) ).is_ok());
            Ok(())
        } 
        else {
            Err(std::io::Error::from(std::io::ErrorKind::NotConnected))
        }
    }
}


// Extra catch if conditional compilation logic becomes broken
// This file must only be included in test builds
#[cfg( not(test) )]
compile_error!("For unit tests ONLY!");
