// lib.rs
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! A basic logger that only writes to hard drive when there is useful data
//!
//! PragmaticLogger buffers log messages in a circular buffer.
//! Only if an important message is recieved is the circular buffer 
//! written to disk. In addition, if the application crasses,
//! the current log buffer will be written to disk.
//! 
//! 
//! 
//! 
//! # Quick start
//! The PragmaticLogger is used via the dependancy injection pattern.
//! In order to have the logger dump the logs on an application crash,
//! The logger must be the main thread. IE it can not be a child thread 
//! of the thread that crashed.
//! 
//! 
//! Clone PragmaticLogger beside your repo
//! Add 
//! `pragmatic_logger = { path = "../pragmatic_logger", version = "<semver>" }` 
//! to Cargo.toml
//! 
//! ` ` `
//! use rand::prelude::*;
//!
//! if rand::random() { // generates a boolean
//!     // Try printing a random unicode code point (probably a bad idea)!
//!     println!("char: {}", rand::random::<char>());
//! }
//!
//! let mut rng = rand::thread_rng();
//! let y: f64 = rng.gen(); // generates a float between 0 and 1
//!
//! let mut nums: Vec<i32> = (1..100).collect();
//! nums.shuffle(&mut rng);
//! ` ` `


mod log_common;
mod log_reciever;
pub mod log_sender;




pub use log_common::Level;
pub use log_reciever::BufferSize;

pub fn build_logger(
    log_file_path: &str,
    store_log_level: log_common::Level,
    dump_log_level: log_common::Level,
    buffer_size: BufferSize,
) -> Result<log_sender::LogSender, &'static str> {
    let fp = std::path::PathBuf::from(log_file_path);

    if fp.is_dir() {
        Err("File path should be a plain file, not a directory")
    } else if fp.parent().is_none() {
        Err("Log file location does not seem to be valid. Are you trying to write to root?")
    } else if store_log_level < dump_log_level {
        Err("Must satisfy store_log_level >= dump_log_level")
    } else if false == buffer_size.is_valid(){
        Err("Specified buffer_size is not a supported value. Must be of type BufferSize")
    }
    else {
        let (sender, receiver) = std::sync::mpsc::channel::<log_common::LogData>();

        if log_common::Level::Off != dump_log_level && log_common::Level::Off != store_log_level {
            // Only bother to spawn reciever if data is going to be logged
            // TODO Will sender still be sending log messages to mpsc?
            let _log_rx_handle = log_reciever::spawn(dump_log_level, receiver, fp, buffer_size);
            debug_assert!( _log_rx_handle.is_some() );
        }

        let log_tx = log_sender::LogSender::new(sender, store_log_level);
        Ok(log_tx)
    }
}
