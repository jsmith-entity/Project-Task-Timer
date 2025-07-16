use ratatui::{prelude::Stylize, style::Color, text::Line};

use strum_macros::{Display, EnumIter};

use crate::task_timer::time_stamp::TimeStamp;

#[derive(EnumIter, Display, Clone)]
pub enum LogType {
    #[strum(to_string = "INFO")]
    INFO,
    #[strum(to_string = "ERROR")]
    ERROR,
}

impl LogType {
    pub fn title(self) -> Line<'static> {
        return format!("{self}").fg(Color::Blue).into();
    }

    fn color(&self) -> Color {
        return match self {
            LogType::INFO => Color::Blue,
            LogType::ERROR => Color::Red,
        };
    }
}

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

        if self.log_arr.len() >= 20 {
            self.log_arr.remove(0);
        }
    }

    pub fn recent(&self) -> Vec<LogRecord> {
        const SIZE: usize = 4;

        let recent_log = if self.log_arr.len() >= SIZE {
            &self.log_arr[self.log_arr.len() - SIZE..]
        } else {
            &self.log_arr[..]
        };

        return recent_log.to_vec();
    }
}
