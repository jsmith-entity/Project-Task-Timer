use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    prelude::{Constraint, Direction, Layout, Rect, Stylize},
    style::Color,
    symbols,
    text::Line,
    widgets::{Block, Padding, Tabs},
};

use serde::{Deserialize, Serialize};
use std::time::Duration;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::{
    logger::{LogType, Logger},
    node::Node,
    popup::Popup,
    views::{controls::*, logger::*, task_status::*, tasks::*, timers::*},
};

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
    pub title: String,
    pub content_height: u16,
    pub content_tree: Node,

    pub timers: TimerView,
    pub task_list: TaskView,
    #[serde(skip)]
    pub task_status: TaskStatus,
    #[serde(skip)]
    logger: Logger,
    pub controls: ControlView,
    #[serde(skip)]
    pub log: LoggerView,
    #[serde(skip)]
    markdown_area_bounds: Rect,

    selected_tab: SelectedTab,
    #[serde(skip)]
    popup: Option<Popup>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            title: "???".to_string(),
            content_height: 0,
            content_tree: Node::new(),
            timers: TimerView::new(),
            task_list: TaskView::new(),
            task_status: TaskStatus::new(),

            logger: Logger::new(),
            controls: ControlView::new(),
            log: LoggerView::new(),
            markdown_area_bounds: Rect::new(0, 0, 0, 0),

            selected_tab: SelectedTab::Tab1,
            popup: None,
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        use Constraint::{Length, Min};
        let area = frame.area();

        let vertical = Layout::vertical([Length(1), Min(0)]);
        let [header_area, body_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        self.render_tabs(frame, tabs_area);

        let title = self.title.clone().bold();
        frame.render_widget(title, title_area);

        let block = Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(Color::Gray);

        frame.render_widget(&block, body_area);
        let inner_area = block.inner(body_area);

        match self.selected_tab {
            SelectedTab::Tab1 => self.draw_task_window(frame, inner_area),
            SelectedTab::Tab2 => self.draw_log_window(frame, inner_area),
            SelectedTab::Tab3 => self.draw_control_window(frame, inner_area),
        }

        if self.popup.is_some() {
            self.popup.as_ref().unwrap().render(frame, area);
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

    fn draw_task_window(&mut self, frame: &mut Frame, area: Rect) {
        self.markdown_area_bounds = area;

        let areas = Layout::new(
            Direction::Horizontal,
            [Constraint::Length(13), Constraint::Length(30), Constraint::Min(0)],
        )
        .split(area);

        let content = &self.content_tree;
        let (task_height, drawn_data) = self.task_list.draw(frame, &areas[1], content);
        let time_height = self.timers.draw(frame, &areas[0], &content, &drawn_data);
        assert!(task_height == time_height);

        let active_times = self.timers.active_time_lines();
        self.task_status
            .render(frame, areas[2], content, &drawn_data, &active_times);

        self.content_height = task_height;
    }

    fn draw_control_window(&mut self, frame: &mut Frame, area: Rect) {
        self.controls.draw(frame, &area);
    }

    fn draw_log_window(&mut self, frame: &mut Frame, area: Rect) {
        self.log.draw(frame, &area);
    }
}

impl Window {
    pub fn handle_events(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('1') => self.selected_tab = SelectedTab::Tab1,
            KeyCode::Char('2') => self.selected_tab = SelectedTab::Tab2,
            KeyCode::Char('3') => self.selected_tab = SelectedTab::Tab3,
            _ => {}
        }
    }

    pub fn toggle_headings(&mut self, visible: bool) {
        self.task_list.toggle_nodes(visible);
        self.task_list.selected_line = 1;
        self.timers.selected_line = 1;
    }

    pub fn log(&mut self, message: &str) {
        self.logger.log(message, LogType::INFO);
        self.log.recent_log = self.logger.recent();
    }

    pub fn enable_popup(&mut self, message: &str) {
        let new_popup = Popup::new(message.to_string());
        self.popup = Some(new_popup);
    }

    pub fn disable_popup(&mut self) {
        self.popup = None;
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
}
