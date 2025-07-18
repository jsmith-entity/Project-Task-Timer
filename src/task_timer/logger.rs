use crate::task_timer::{log_type::LogType, time_stamp::TimeStamp};

#[derive(Clone)]
pub struct LogRecord {
    pub log_type: LogType,
    pub time_stamp: TimeStamp,
    pub message: String,
}

#[derive(Default)]
pub struct Logger {
    log_arr: Vec<LogRecord>,
}

impl Logger {
    pub fn new() -> Self {
        return Self { log_arr: Vec::new() };
    }

    pub fn log(&mut self, message: &str, log_type: LogType) {
        self.log_arr.push(LogRecord {
            log_type,
            time_stamp: TimeStamp::new(),
            message: message.to_string(),
        });

        if self.log_arr.len() >= 40 {
            self.log_arr.remove(0);
        }
    }

    pub fn recent(&self) -> Vec<LogRecord> {
        const SIZE: usize = 15;

        let recent_log = if self.log_arr.len() >= SIZE {
            &self.log_arr[self.log_arr.len() - SIZE..]
        } else {
            &self.log_arr[..]
        };

        return recent_log.to_vec();
    }
}
