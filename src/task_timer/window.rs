use crossterm::event::KeyCode;
use ratatui::prelude::{Constraint, Direction, Layout, Rect, Stylize};
use ratatui::style::Color;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Padding, Tabs};
use ratatui::{Frame, symbols};
use std::time::Duration;

use ratatui::prelude::Constraint::{Length, Min};

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::task_timer::logger::Logger;
use crate::task_timer::node::Node;
use crate::task_timer::views::{controls::*, logger::*, tasks::*, timers::*};

#[derive(Serialize, Deserialize, EnumIter, Display, Clone, Copy)]
enum SelectedTab {
    #[strum(to_string = "Main")]
    Tab1,
    #[strum(to_string = "Log")]
    Tab2,
    #[strum(to_string = "Controls")]
    Tab3,
}

impl SelectedTab {
    fn title(self) -> Line<'static> {
        return format!("  {self}  ").fg(Color::Gray).into();
    }
}

#[derive(Serialize, Deserialize)]
pub struct Window {
    pub file_name: String,
    pub content_height: u16,
    pub content_tree: Node,

    pub timers: TimerView,
    pub task_list: TaskView,
    #[serde(skip)]
    logger: Logger,
    pub controls: ControlView,
    #[serde(skip)]
    pub log: LoggerView,
    #[serde(skip)]
    markdown_area_bounds: Rect,

    selected_tab: SelectedTab,
}

impl Window {
    pub fn new() -> Self {
        Self {
            file_name: "???".to_string(),
            content_height: 0,
            content_tree: Node::new(),
            task_list: TaskView::new(),
            timers: TimerView::new(),

            logger: Logger::new(),
            controls: ControlView::new(),
            log: LoggerView::new(),
            markdown_area_bounds: Rect::new(0, 0, 0, 0),

            selected_tab: SelectedTab::Tab1,
        }
    }

    pub fn handle_events(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('1') => self.selected_tab = SelectedTab::Tab1,
            KeyCode::Char('2') => self.selected_tab = SelectedTab::Tab2,
            KeyCode::Char('3') => self.selected_tab = SelectedTab::Tab3,
            _ => {}
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let vertical = Layout::vertical([Length(1), Min(0)]);
        let [header_area, body_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        self.render_tabs(frame, tabs_area);

        let block = Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(Color::Gray);

        frame.render_widget(&block, body_area);
        let inner_area = block.inner(body_area);

        // TODO: draw heading - tab selected
        match self.selected_tab {
            SelectedTab::Tab1 => self.draw_task_window(frame, inner_area),
            SelectedTab::Tab2 => self.draw_log_window(frame, inner_area),
            SelectedTab::Tab3 => self.draw_control_window(frame, inner_area),
        }
    }

    pub fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let titles = SelectedTab::iter().map(SelectedTab::title);

        let selected_tab_idx = self.selected_tab as usize;
        let highlight_style = (Color::Black, Color::Gray);

        let erm = Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_idx)
            .padding("", "")
            .divider(" ");

        frame.render_widget(erm, area);
    }

    pub fn select_line(&mut self, line_num: u16) {
        let area_bounds = self.markdown_area_bounds;

        let win_max_height = area_bounds.y + area_bounds.height;

        let lower_bound = area_bounds.y;
        let upper_bound = if self.content_height < win_max_height {
            self.content_height + 1
        } else {
            win_max_height
        };

        let within_bounds = line_num >= lower_bound && line_num < upper_bound;
        if within_bounds {
            self.task_list.selected_line = line_num;
            self.timers.selected_line = line_num;
        }
    }

    pub fn update_time(&mut self) {
        let node_data = self.timers.active_times();

        for entry in node_data.iter() {
            let node = self.content_tree.get_node(&entry.node_path).unwrap();

            node.content_times[entry.task_num] += Duration::from_secs(1);
        }
    }

    pub fn update_completed_task(&mut self) {
        if let Some((task_idx, found_path)) = self.task_list.selected_task() {
            let node = self.content_tree.get_node(found_path).unwrap();

            // stop a timer if it exists
            if self.timers.active_on_selected() {
                self.timers.stop_selected_time();
            }

            node.completed_tasks[task_idx] = !node.completed_tasks[task_idx];
        }
    }

    pub fn toggle_headings(&mut self, visible: bool) {
        self.task_list.toggle_nodes(visible);
        self.task_list.selected_line = 1;
        self.timers.selected_line = 1;
    }

    pub fn log(&mut self, message: &str) {
        self.logger.log(message);
        self.log.recent_log = self.logger.recent();
    }

    fn draw_task_window(&mut self, frame: &mut Frame, area: Rect) {
        self.markdown_area_bounds = area;

        let areas = Layout::new(
            Direction::Horizontal,
            [Constraint::Length(13), Constraint::Min(0)],
        )
        .split(area);

        let content = &self.content_tree;
        let (task_height, drawn_data) = self.task_list.draw(frame, &areas[1], content);
        let time_height = self.timers.draw(frame, &areas[0], &content, &drawn_data);
        assert!(task_height == time_height);

        self.content_height = task_height;
    }

    fn draw_control_window(&mut self, frame: &mut Frame, area: Rect) {
        self.controls.draw(frame, &area);
    }

    fn draw_log_window(&mut self, frame: &mut Frame, area: Rect) {
        self.log.draw(frame, &area);
    }
}
