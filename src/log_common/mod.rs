// log common module

/// Get current time from Chrono in Local timezone
pub fn get_time_now() -> chrono::DateTime<chrono::offset::Local> {
    chrono::offset::Local::now()
}

/// Add padding to string keep text alligned
/// 
/// Appends spaces to make line the target length
fn pad_string(target_length: usize, line: &mut String) {
    const PADDING_CHAR: char = ' '; // single space
    const PADDING_CHAR_3: &str = "   "; // three spaces

    if line.len() < target_length {
        let delta = target_length - line.len();
        if 3 == delta {
            // Special handling for three spaces which is the common case
            *line += PADDING_CHAR_3;
        } else {
            for _i in 0..delta {
                line.push(PADDING_CHAR);
            }
        }
    }
}

#[derive(Clone)]
pub struct LogData {
    level: Level,
    time_stamp: chrono::DateTime<chrono::offset::Local>,
    line: String,
}

impl LogData {

    /// Construct new log message
    pub fn new(
        level: Level,
        time_stamp: chrono::DateTime<chrono::offset::Local>,
        line: String,
    ) -> Self {
        Self {
            level,
            time_stamp,
            line,
        }
    }

    /// Read level of this log message
    pub fn level(&self) -> Level {
        self.level
    }

    /// Read log message formatted into String
    /// 
    /// Output = "<timestamp> | <log level> | <payload line>"
    pub fn as_string(&self) -> String {
        const EXPECTED_DATE_LENGTH: usize = 35;
        let mut time_stamp = self.time_stamp.to_rfc3339();
        debug_assert!(time_stamp.len() <= EXPECTED_DATE_LENGTH);
        pad_string(EXPECTED_DATE_LENGTH, &mut time_stamp);
        format!("{} | {} | {}\n", time_stamp, self.level, self.line) // return formatted string
    }
}

/// Support log levels
#[derive(PartialEq, std::cmp::PartialOrd, Clone, Copy)]
pub enum Level {
    Trace = 4,
    Info  = 3,
    Warn  = 2,
    Error = 1,
    Off   = 0,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Trace => write!(f, "Trace"),
            Self::Info  => write!(f, "Info "),
            Self::Warn  => write!(f, "Warn "),
            Self::Error => write!(f, "Error"),
            Self::Off   => write!(f, "Off  "),
        }
    }
}
