
//! PragmaticLogger
//! 
//! A basic buffered logger that only writes to hard drive when there is important data.
//!
//! This is only intended as an experimental project, and not for production use. 
//! 
//! The goal being to optimize performance and hard-drive wear by only writing logs when something has failed.
//! By buffering past messages, the information leading up to the failure can be captured.
//! Additionally by having the logger in its own thread, the messages leading to a crash can also be recorded.
//! 
//! PragmaticLogger buffers log messages in a circular buffer.
//! Only if an important message is received is the circular buffer 
//! written to disk. In addition, if the application crashes,
//! the current log buffer will be written to disk.
//! 
//! 
//! # License
//! 
//! Licensed under the Apache License, Version 2.0 LICENSE-APACHE or
//! <https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//! LICENSE-MIT or <https://opensource.org/licenses/MIT>, at your
//! option.
//! 
//! THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//! 
//! # To get started
//! 
//! See `README.md` for information on using this project.
//! 
//!

mod log_common;
mod log_receiver;

pub mod log_sender;
pub use log_common::Level;
pub use log_receiver::BufferSize;


/// Construct a new log sender and receiver pair
/// 
/// ## Params
/// 
/// **log_file_path**: &str
/// 
/// Path to output text file.
/// such as `"./my_log.txt"` or `"./my_log.log"` 
/// 
/// **store_log_level**: [`Level`]
/// 
/// Messages this severe and more sever will be buffered when sent.
/// Messages less severe than this level will be dropped.
/// 
/// For example:   
/// store_log_level = Warn   
/// Will drop messages Trace or Info,   
/// and buffer messages of type Warn, Error   
/// 
/// **dump_log_level**: [`Level`]
/// 
/// Messages this severe and more severe will cause buffer to be dumped (written) to file.
/// 
/// For example:   
/// dump_log_level = Warn   
/// Will dump buffer to file if message is Warn or Error   
/// 
/// **buffer_size**: [`BufferSize`]
/// 
/// Specify the size of the buffer in messages.
/// This is effectively the number of messages of history
/// that will be included when an error occurs.
/// 
/// 
/// # Example
/// 
/// ```rust
/// 
/// use pragmatic_logger::{log_sender::LogSender, Level, BufferSize, build_logger};
/// 
/// // Some program that does fascinating stuff
/// fn my_program(log: LogSender){
///     log.info_str("Running my program");
///     // do stuff
/// }
/// 
/// // Code to launch program
/// fn main() -> Result::<(),String>{
/// 
///     const LOG_LOCATION : &'static str = "/media/ramdisk/my_program_log.txt";
/// 
///     // Make logger instance
///     let log = build_logger(LOG_LOCATION, Level::Trace, Level::Warn, BufferSize::Size128)?;
///     log.info_str("Running");
/// 
///     // Launch my program
///     let my_program_logger = log.clone();
///     let handle = std::thread::spawn(move || {
///         my_program( my_program_logger );
///     });
/// 
///     // Wait for exit and log if thread panicked
///     if let Err(e) = handle.join(){
///         let error_message = format!("my_program returned: {:?}", e);
///         log.error_string(error_message);
///    }
/// 
///     log.info_str("Done");
/// 
///     // Close logger receiver thread in a controlled way
///     log.shutdown();
///     Ok(())
/// }
/// ```

pub fn build_logger(
    log_file_path: &str,
    store_log_level: Level,
    dump_log_level: Level,
    buffer_size: BufferSize,
) -> Result<log_sender::LogSender, &'static str> {
    let fp = std::path::PathBuf::from(log_file_path);

    if fp.is_dir() {
        Err("File path should be a plain file, not a directory")
    } else if fp.parent().is_none() {
        Err("Log file location does not seem to be valid. Are you trying to write to root?")
    } else if store_log_level < dump_log_level {
        Err("Must satisfy store_log_level >= dump_log_level")
    } else if !buffer_size.is_valid(){
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
