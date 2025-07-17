use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    prelude::{Constraint, Layout, Rect, Stylize},
    style::Color,
    symbols,
    text::Line,
    widgets::{Block, Padding, Tabs},
};

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::task_timer::{
    logger::{LogType, Logger},
    node::{Node, NodePath},
    popup::Popup,
    views::{controls::*, home::main_view::*, logger::*},
};

#[derive(Serialize, Deserialize, EnumIter, Display, Clone, Copy, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum RenderedNodeType {
    Heading,
    Task(usize),            // Index of the task in node.content
    ChildHeading(NodePath), // Immediate child heading
}

#[derive(Clone, Debug)]
pub struct RenderedNode {
    pub node_type: RenderedNodeType,
    pub node_path: NodePath,
}

#[derive(Serialize, Deserialize)]
pub struct Window {
    pub title: String,
    pub content_height: u16,
    pub content_tree: Node,

    pub selected_line: u16,
    #[serde(skip)]
    pub display_data: Vec<RenderedNode>,

    displayed_node: Node,
    selected_tab: SelectedTab,

    #[serde(skip)]
    main_view: MainView,
    #[serde(skip)]
    logger: Logger,
    pub controls: ControlView,
    #[serde(skip)]
    pub log: LoggerView,
    #[serde(skip)]
    popup: Option<Popup>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            title: "???".to_string(),
            content_height: 0,
            content_tree: Node::new(),
            display_data: Vec::new(),
            displayed_node: Node::new(),
            selected_line: 1,

            main_view: MainView::new(),

            logger: Logger::new(),
            controls: ControlView::new(),
            log: LoggerView::new(),

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

        self.main_view.update(
            inner_area,
            self.content_tree.clone(),
            self.display_data.clone(),
            self.selected_line,
        );

        self.content_height = self.display_data.len() as u16;

        match self.selected_tab {
            SelectedTab::Tab1 => frame.render_widget(&self.main_view, inner_area),
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

    fn draw_control_window(&mut self, frame: &mut Frame, area: Rect) {
        self.controls.draw(frame, area);
    }

    fn draw_log_window(&mut self, frame: &mut Frame, area: Rect) {
        self.log.draw(frame, area);
    }
}

impl Window {
    pub fn update_tree(&mut self, new_root: Node) {
        self.content_tree = new_root.clone();
        self.displayed_node = self.content_tree.clone();

        self.update_display_data();
    }

    fn update_display_data(&mut self) {
        let res = MainView::collect_display_data(&self.content_tree, &self.displayed_node);
        match res {
            Ok(new_data) => self.display_data = new_data,
            Err(e) => self.log(&e, LogType::ERROR),
        }
    }

    pub fn handle_events(&mut self, key_code: KeyCode) {
        let old_tab = self.selected_tab;

        match key_code {
            KeyCode::Char('1') => self.selected_tab = SelectedTab::Tab1,
            KeyCode::Char('2') => self.selected_tab = SelectedTab::Tab2,
            KeyCode::Char('3') => self.selected_tab = SelectedTab::Tab3,
            _ => {}
        };

        let changed_tabs = if old_tab == self.selected_tab { false } else { true };
        if !changed_tabs {
            match self.selected_tab {
                SelectedTab::Tab1 => self.handle_main_events(key_code),
                SelectedTab::Tab2 => self.handle_log_events(key_code),
                _ => (),
            };
        }
    }

    pub fn log(&mut self, message: &str, log_type: LogType) {
        self.logger.log(message, log_type);
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
        let area_bounds = self.main_view.content_area;

        let lower_bound = area_bounds.y - 1;
        let upper_bound = self.content_height + 1;

        let within_bounds = line_num >= lower_bound && line_num < upper_bound;
        if within_bounds {
            self.selected_line = line_num;
        }
    }

    fn enter_subheading(&mut self) {
        // TODO:update breadcrumb

        if let Some(new_node_path) = self.main_view.get_subheading_path(self.selected_line as usize) {
            if let Some(new_node) = self.content_tree.get_node(&new_node_path) {
                self.displayed_node = new_node.clone();
                self.update_display_data();
            } else {
                self.log(
                    "Failed to convert node path to node when entering subheading",
                    LogType::ERROR,
                );
            }
        } else {
            self.log(
                "Failed to retrieve subheading from display data when entering subheading",
                LogType::ERROR,
            );
        }
    }

    fn handle_main_events(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('j') => self.select_line(self.selected_line + 1),
            KeyCode::Char('k') => self.select_line(self.selected_line - 1),
            // KeyCode::Char('s') => self.timers.try_activate(),
            // KeyCode::Char(' ') => self.update_completed_task(),
            KeyCode::Enter => self.enter_subheading(),
            _ => (),
        };
    }

    fn handle_log_events(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('h') => self.log.prev_filter(),
            KeyCode::Char('l') => self.log.next_filter(),
            _ => (),
        }
    }
}
