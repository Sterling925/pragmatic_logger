//! PragmaticLogger
//! Licensed under the Apache License, Version 2.0 LICENSE-APACHE or
//! <https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//! LICENSE-MIT or <https://opensource.org/licenses/MIT>, at your
//! option.


mod log_common;
mod log_receiver;

pub mod log_sender;
pub use log_common::Level;
pub use log_receiver::BufferSize;

/// A basic buffered logger that only writes to hard drive when there is important data
///
/// PragmaticLogger buffers log messages in a circular buffer.
/// Only if an important message is received is the circular buffer 
/// written to disk. In addition, if the application crashes,
/// the current log buffer will be written to disk.
/// 
/// 
/// # Quick start
/// 
/// The PragmaticLogger is used via the dependency injection pattern.
/// In order to have the logger dump the logs on an application panic,
/// The logger must be the main thread. IE it can not be a child thread 
/// of the thread that crashed.
/// 
/// 
/// Clone PragmaticLogger beside your repo
/// Add 
/// `pragmatic_logger = { path = "../pragmatic_logger", version = "<semver>" }` 
/// to Cargo.toml
/// 
/// ```rust
/// // Some program that does fascinating stuff
/// fn my_program(log: pragmatic_logger::log_sender::LogSender){
///     log.info_str("Running my program");
///     // do stuff
/// }
/// 
/// // Code to launch program
/// fn main() -> Result::<(),String>{
/// 
///     // Make logger instance
///     const LOG_LOCATION : &'static str = "/media/ramdisk/my_program_log.txt";
/// 
///     let log = pragmatic_logger::build_logger(LOG_LOCATION, pragmatic_logger::Level::Trace, pragmatic_logger::Level::Warn, pragmatic_logger::BufferSize::Size128)?;
///     log.info_str("Running");
/// 
/// 
///     // Launch program
///     let my_program_logger = log.clone();
///     let handle = std::thread::spawn(move || {
///         my_program( my_program_logger );
///     });
/// 
///     // Clean up
///     if let Err(e) = handle.join(){
///         let error_message = format!("my_program returned: {:?}", e);
///         log.error_string(error_message);
///    }
/// 
///     log.info_str("Done");
/// 
///     log.shutdown();
///     Ok(())
/// }
/// ```


/// Construct a new log sender and receiver pair
///
///  
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
            // Only bother to spawn receiver if data is going to be logged
            let _log_rx_handle = log_receiver::spawn(dump_log_level, receiver, fp, buffer_size);
            if _log_rx_handle.is_none(){
                return Err("Failed to spawn receiver thread");
            }
        }

        let log_tx = log_sender::LogSender::new(sender, store_log_level);
        Ok(log_tx)
    }
}
