use crate::task_timer::time_stamp::TimeStamp;

use crate::task_timer::time_stamp::*;

pub struct Logger {
    messages: Vec<LogRecord>,
}

impl Logger {
    pub fn new() -> Self {
        return Self { messages: Vec::new() };
    }

    pub fn log(&mut self, message: &str) {
        self.messages.push((TimeStamp::new(), message.to_string()));

        if self.messages.len() >= 20 {
            self.messages.remove(0);
        }
    }

    pub fn recent(&self) -> Vec<LogRecord> {
        const SIZE: usize = 4;

        let recent_log = if self.messages.len() >= SIZE {
            &self.messages[self.messages.len() - SIZE..]
        } else {
            &self.messages[..]
        };

        return recent_log.to_vec();
    }
}
