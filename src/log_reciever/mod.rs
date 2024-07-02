// log reciever module

// use std::thread;
use super::log_common;

pub mod circular_buffer;


// const BUFFER_SIZE: usize = 0x0200;
const POLLING_RECV_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(500);


#[derive(Debug, Copy, Clone)]
#[repr(usize)]
pub enum BufferSize{
    Size16   = 16_usize,
    Size32   = 32_usize,
    Size64   = 64_usize,
    Size128  = 128_usize,
    Size256  = 256_usize,
    Size512  = 512_usize,
    Size1024 = 1024_usize,
    Size2048 = 2048_usize,
}

impl BufferSize{
  pub fn value(&self) -> usize{
      return (*self).clone() as usize;
  }
  pub fn is_valid(&self) -> bool{
      let v = (*self).clone() as usize;
      let mut ans = true;
      if 1 != v.count_ones(){
          ans = false;
      }
      if 16 > v{
          ans = false;
      }
      if 2048 < v{
          ans = false;
      }
      return ans;
  }
}


pub fn spawn(
    log_dump_level: log_common::Level,
    reciever: std::sync::mpsc::Receiver<log_common::LogData>,
    log_file_path: std::path::PathBuf,
    buffer_size: BufferSize,
) -> Option<std::thread::JoinHandle<()>> {
    if log_common::Level::Off != log_dump_level {
        debug_assert!( buffer_size.is_valid() );

        let mut buffer = std::vec::Vec::<String>::with_capacity(buffer_size.value());
        for _k in 0..buffer_size.value() {
            buffer.push(String::new());
        }

        let text_data_writer = circular_buffer::data_writer::DataWriter::new();
        let circle = circular_buffer::CircularStringsBuffer::new(buffer, text_data_writer);

        Some(std::thread::spawn(move || {    
            let mut logger = LogReciever::new(log_dump_level, reciever, log_file_path, circle);
            logger.execute();
        })) // returns thread handle

    } else {
        None
    }
}

struct LogReciever<T: circular_buffer::TextDataWriter + Send> {
    log_dump_level: log_common::Level,
    reciever: std::sync::mpsc::Receiver<log_common::LogData>,
    log_file_path: std::path::PathBuf,
    buffer: circular_buffer::CircularStringsBuffer::<T>,
}

impl<T: circular_buffer::TextDataWriter + Send> LogReciever<T> {
    fn new(
        log_dump_level: log_common::Level,
        reciever: std::sync::mpsc::Receiver<log_common::LogData>,
        log_file_path: std::path::PathBuf,
        buffer: circular_buffer::CircularStringsBuffer::<T>,
    ) -> Self {
        Self {
            log_dump_level,
            reciever,
            log_file_path,
            buffer,
        }
    }

    fn execute(&mut self) {
        debug_assert!(log_common::Level::Off != self.log_dump_level); // execute should not be called if log_dump_level is Off
        loop {
            let msg = self.reciever.recv_timeout(POLLING_RECV_TIMEOUT);
            if let Ok(payload) = msg {
                self.buffer.push(payload.as_string());

                if payload.level() > self.log_dump_level {
                    // NOP for common case
                } else if payload.level() == log_common::Level::Off {
                    break;
                } else if payload.level() <= self.log_dump_level {
                    self.dump();
                }
            } else if let Err(e) = msg {
                match e {
                    std::sync::mpsc::RecvTimeoutError::Timeout => (), // On timeout, just go around for another try
                    std::sync::mpsc::RecvTimeoutError::Disconnected => {
                        let t = log_common::get_time_now().to_rfc3339();
                        self.buffer.push(format!(
                            "{} | Error | Ending logger thread due to MPSC Disconnected",
                            t
                        ));
                        self.dump();
                        break;
                    }
                }
            } else {
                unreachable!();
            }
        }
    }

    fn dump(&mut self) {
        let _r = self.buffer.write_to_file_and_clear(&self.log_file_path);
        #[cfg(debug_assertions)]
        if let Err(e) = _r {
            println!("Error: write_to_file_and_clear() returned: {}", e);
        }
    }
}
