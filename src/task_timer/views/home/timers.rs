use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

use std::time::Duration;

use crate::task_timer::node::Node;

#[derive(Default, Clone)]
pub struct Timers {
    pub total_time: Duration,
    pub times: Vec<Duration>, // Positions correspond to the nodes content (task) array entries
    pub selected_line: u16,
    pub content_height: u16,
}

impl Timers {
    pub fn new(node: &Node) -> Self {
        let total_time = node.total_time.clone();
        let selected_line = 1;

        let mut times = node.content_times.clone();
        let mut subheadings_times = Timers::subheading_times(node);
        times.append(&mut subheadings_times);

        let content_height = times.len() as u16;

        return Self {
            total_time,
            times,
            selected_line,
            content_height,
        };
    }

    fn subheading_times(node: &Node) -> Vec<Duration> {
        let mut times: Vec<Duration> = Vec::new();
        for subheading in node.children.iter() {
            let total_time = Timers::total_time(&subheading);
            times.push(total_time);
        }

        return times;
    }

    fn total_time(node: &Node) -> Duration {
        let task_time: Duration = node.content_times.iter().sum();
        let mut subheading_time = Duration::default();
        for subheading in node.children.iter() {
            subheading_time += Timers::total_time(subheading);
        }

        return task_time + subheading_time;
    }

    fn format_duration(duration: &Duration) -> String {
        let secs = duration.as_secs();
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;
        format!("[{:02}:{:02}:{:02}]", hours, minutes, seconds)
    }
}

impl Widget for &Timers {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for (idx, time) in self.times.iter().enumerate() {
            let mut style = Style::default();
            if idx as u16 + 1 == self.selected_line {
                style = style.fg(Color::Black).bg(Color::Gray);
            }

            let display_area = Rect {
                x: area.x,
                y: area.y + idx as u16,
                width: area.width,
                height: 1,
            };

            Line::from(Timers::format_duration(time))
                .style(style)
                .render(display_area, buf);
        }
    }
}
