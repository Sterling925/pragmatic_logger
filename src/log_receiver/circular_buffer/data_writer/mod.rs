
pub trait TextDataWriter {
    fn open(&mut self, p: &std::path::Path) -> Result<(), std::io::Error>;
    fn close(&mut self);
    fn write(&mut self, line: &str) -> Result<(), std::io::Error>;
}

mod writer;


pub use writer::DataWriter as DataWriter;


