// log sender module

use super::log_common;

#[derive(Clone)]
pub struct LogSender {
    log_level: log_common::Level,
    sender: std::sync::mpsc::Sender<log_common::LogData>,
}


/// LogSender is an object for sending new messages to be buffered
impl LogSender {
    pub(crate) fn new(
        sender: std::sync::mpsc::Sender<log_common::LogData>,
        log_level: log_common::Level,
    ) -> Self {
        Self { sender, log_level }
    }

    /// Construct and send massage if valid `level`
    fn construct_and_send(&self, level: log_common::Level, line: String) {
        if level <= self.log_level {
            let d = log_common::LogData::new(level, log_common::get_time_now(), line);

            let _ret = self.sender.send(d);
            debug_assert!(_ret.is_ok());
        } // else, drop message
    }

    /// Close logging thread
    /// 
    /// Sends command to set log level to `Off`,
    /// which signals thread to exit.
    /// code waits for mpsc to disconnect before returning.
    /// 
    /// This method should be called before program exit.
    pub fn shutdown(&self) {
        const DELAY: std::time::Duration = std::time::Duration::from_millis(100);

        let off_command = log_common::LogData::new(
            log_common::Level::Off,
            log_common::get_time_now(),
            String::new(),
        );

        // Wait up to 500 * DELAY for MPSC connection to drop signaling the reciever has closed.
        for _s in 0..500 {
            // Per: https://doc.rust-lang.org/std/sync/mpsc/struct.SendError.html
            // A send operation can only fail if the receiving end of a channel is disconnected,
            // implying that the data could never be received.
            // The error contains the data being sent as a payload so it can be recovered.
            if self.sender.send(off_command.clone()).is_ok() {
                std::thread::sleep(DELAY);
            } else {
                // Connection has dropped. Stop waiting.
                break;
            }
        }
    }

    /// Log a `Trace` level message
    ///
    /// Message will only be sent to buffer if 
    /// log threshold configured to allow this level.
    ///
    /// # Panic
    /// 
    /// Will panic if built in debug mode and MSPC send fails
    ///
    #[allow(dead_code)]
    pub fn trace_str(&self, line: &str) {
        self.construct_and_send(log_common::Level::Trace, line.to_string());
    }


    /// Log a `Trace` level message
    ///
    /// Message will only be sent to buffer if 
    /// log threshold configured to allow this level.
    ///
    /// # Panic
    /// 
    /// Will panic if built in debug mode and MSPC send fails
    ///
    #[allow(dead_code)]
    pub fn trace_string(&self, line: String) {
        self.construct_and_send(log_common::Level::Trace, line);
    }


    /// Log a `Info` level message
    ///
    /// Message will only be sent to buffer if 
    /// log threshold configured to allow this level.
    ///
    /// # Panic
    /// 
    /// Will panic if built in debug mode and MSPC send fails
    ///
    #[allow(dead_code)]
    pub fn info_str(&self, line: &str) {
        self.construct_and_send(log_common::Level::Info, line.to_string());
    }

    /// Log a `Info` level message
    ///
    /// Message will only be sent to buffer if 
    /// log threshold configured to allow this level.
    ///
    /// # Panic
    /// 
    /// Will panic if built in debug mode and MSPC send fails
    ///
    #[allow(dead_code)]
    pub fn info_string(&self, line: String) {
        self.construct_and_send(log_common::Level::Info, line);
    }


    /// Log a `Warn` level message
    ///
    /// Message will only be sent to buffer if 
    /// log threshold configured to allow this level.
    ///
    /// # Panic
    /// 
    /// Will panic if built in debug mode and MSPC send fails
    ///
    #[allow(dead_code)]
    pub fn warn_str(&self, line: &str) {
        self.construct_and_send(log_common::Level::Warn, line.to_string());
    }

    /// Log a `Warn` level message
    ///
    /// Message will only be sent to buffer if 
    /// log threshold configured to allow this level.
    ///
    /// # Panic
    /// 
    /// Will panic if built in debug mode and MSPC send fails
    ///
    #[allow(dead_code)]
    pub fn warn_string(&self, line: String) {
        self.construct_and_send(log_common::Level::Warn, line);
    }

    /// Log a `Error` level message
    ///
    /// Message will only be sent to buffer if 
    /// log threshold configured to allow this level.
    ///
    /// # Panic
    /// 
    /// Will panic if built in debug mode and MSPC send fails
    ///
    #[allow(dead_code)]
    pub fn error_str(&self, line: &str) {
        self.construct_and_send(log_common::Level::Error, line.to_string());
    }

    /// Log a `Error` level message
    ///
    /// Message will only be sent to buffer if 
    /// log threshold configured to allow this level.
    ///
    /// # Panic
    /// 
    /// Will panic if built in debug mode and MSPC send fails
    ///
    #[allow(dead_code)]
    pub fn error_string(&self, line: String) {
        self.construct_and_send(log_common::Level::Error, line);
    }

}
