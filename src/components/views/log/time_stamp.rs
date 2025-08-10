use chrono::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimeStamp {
    day: u32,
    month: String,
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl TimeStamp {
    pub fn new() -> Self {
        let current_time = Local::now();
        return Self {
            day: current_time.day(),
            month: current_time.format("%B").to_string(),
            hours: current_time.hour(),
            minutes: current_time.minute(),
            seconds: current_time.second(),
        };
    }

    pub fn print(&self) -> String {
        return format!(
            "{} {}: {:02}:{:02}:{:02}",
            self.day, self.month, self.hours, self.minutes, self.seconds
        );
    }
}
