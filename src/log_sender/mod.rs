// log sender module

use super::log_common;

#[derive(Clone)]
pub struct LogSender {
    log_level: log_common::Level,
    sender: std::sync::mpsc::Sender<log_common::LogData>,
}

impl LogSender {
    pub(crate) fn new(
        sender: std::sync::mpsc::Sender<log_common::LogData>,
        log_level: log_common::Level,
    ) -> Self {
        Self { sender, log_level }
    }

    fn construct_and_send(&self, level: log_common::Level, line: String) {
        if level <= self.log_level {
            let d = log_common::LogData::new(level, log_common::get_time_now(), line);

            let _ret = self.sender.send(d);
            debug_assert!(_ret.is_ok());
        } // else, drop message
    }

    // Close logging thread
    pub fn shutdown(&self) {
        const DELAY: std::time::Duration = std::time::Duration::from_millis(100);

        let off_command = log_common::LogData::new(
            log_common::Level::Off,
            log_common::get_time_now(),
            String::new(),
        );

        // Wait up to 500 * DELAY for MPSC connection to drop signaling the reciever has closed.
        for _s in 0..500 {
            /*
            Per: https://doc.rust-lang.org/std/sync/mpsc/struct.SendError.html
            A send operation can only fail if the receiving end of a channel is disconnected,
            implying that the data could never be received.
            The error contains the data being sent as a payload so it can be recovered.
            */
            if self.sender.send(off_command.clone()).is_ok() {
                std::thread::sleep(DELAY);
            } else {
                // Connection has dropped. Stop waiting.
                break;
            }
        }
    }

    #[allow(dead_code)]
    pub fn trace_str(&self, line: &str) {
        self.construct_and_send(log_common::Level::Trace, line.to_string());
    }

    #[allow(dead_code)]
    pub fn trace_string(&self, line: String) {
        self.construct_and_send(log_common::Level::Trace, line);
    }

    #[allow(dead_code)]
    pub fn info_str(&self, line: &str) {
        self.construct_and_send(log_common::Level::Info, line.to_string());
    }

    #[allow(dead_code)]
    pub fn info_string(&self, line: String) {
        self.construct_and_send(log_common::Level::Info, line);
    }

    #[allow(dead_code)]
    pub fn warn_str(&self, line: &str) {
        self.construct_and_send(log_common::Level::Warn, line.to_string());
    }

    #[allow(dead_code)]
    pub fn warn_string(&self, line: String) {
        self.construct_and_send(log_common::Level::Warn, line);
    }

    #[allow(dead_code)]
    pub fn error_str(&self, line: &str) {
        self.construct_and_send(log_common::Level::Error, line.to_string());
    }

    #[allow(dead_code)]
    pub fn error_string(&self, line: String) {
        self.construct_and_send(log_common::Level::Error, line);
    }
}
