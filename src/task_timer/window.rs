use crossterm::event::KeyCode;
use ratatui::{
    prelude::{Buffer, Constraint, Layout, Rect, Stylize},
    style::Color,
    symbols,
    text::Line,
    widgets::{Block, Padding, Tabs, Widget},
};

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::task_timer::{
    log_type::*,
    node::Node,
    popups::*,
    session_manager::SessionState,
    traits::*,
    views::{controls::*, log::log_view::*, task_view::task_view::TaskView},
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

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);

        let selected_tab_idx = self as usize;
        let highlight_style = (Color::Black, Color::Gray);

        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_idx)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Window {
    pub title: String,

    selected_tab: SelectedTab,

    task_view: TaskView,
    logger: LogView,
    #[serde(skip)]
    controls: Controls,
    popup: PopupType,
}

impl Window {
    pub fn new() -> Self {
        Self {
            title: "???".to_string(),

            task_view: TaskView::new(),

            controls: Controls::new(),
            logger: LogView::new(),

            selected_tab: SelectedTab::Tab1,
            popup: PopupType::None,
        }
    }

    pub fn load(&mut self, window: Window) {
        self.title = window.title;

        self.task_view = TaskView::new_with(window.task_view);

        self.controls = Controls::new();
        self.logger = window.logger;

        self.selected_tab = window.selected_tab;
        self.popup = window.popup;
    }

    pub fn log(&mut self, message: &str, log_type: LogType) {
        self.logger.log(message, log_type);
    }

    pub fn update(&mut self) {
        self.task_view.update();
    }

    pub fn update_tree(&mut self, new_root: Node) {
        self.task_view.root_node = new_root.clone();
        self.task_view.update_display_data(new_root);
    }

    pub fn update_time(&mut self) {
        if let Err(e) = self.task_view.update_time() {
            self.log(&e, LogType::ERROR);
        }
    }

    pub fn extract_node(&self) -> Node {
        return self.task_view.root_node.clone();
    }
}

impl EventHandler for Window {
    fn handle_events(&mut self, key_code: KeyCode) -> SessionState {
        let new_state: SessionState;

        if self.popup != PopupType::None {
            new_state = self.popup.handle_events(key_code);
            if new_state != SessionState::AwaitingPrompt {
                self.popup = PopupType::None;
            }
        } else {
            new_state = match key_code {
                KeyCode::Char('1') => {
                    self.selected_tab = SelectedTab::Tab1;
                    SessionState::Running
                }
                KeyCode::Char('2') => {
                    self.selected_tab = SelectedTab::Tab2;
                    SessionState::Running
                }
                KeyCode::Char('3') => {
                    self.selected_tab = SelectedTab::Tab3;
                    SessionState::Running
                }
                KeyCode::Esc => {
                    self.popup = PopupType::ConfirmQuit;
                    SessionState::AwaitingPrompt
                }
                _ => SessionState::Running,
            };

            let res = match self.selected_tab {
                SelectedTab::Tab1 => self.task_view.handle_events(key_code),
                SelectedTab::Tab2 => self.logger.handle_events(key_code),
                _ => Ok((InfoSubType::None, "erm".to_string())),
            };
            match res {
                Ok((log_type, info)) => {
                    if log_type != InfoSubType::None {
                        self.log(&log_type.message(info), LogType::INFO(log_type));
                    }
                }
                Err(e) => self.log(&e, LogType::ERROR),
            }
        }

        return new_state;
    }
}

impl Widget for &Window {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};

        let vertical = Layout::vertical([Length(1), Min(0)]);
        let [header_area, body_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        self.selected_tab.render(tabs_area, buf);

        Line::from(self.title.clone()).bold().render(title_area, buf);

        let block = Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(Color::Gray);
        let inner_area = block.inner(body_area);
        block.render(body_area, buf);

        match self.selected_tab {
            SelectedTab::Tab1 => self.task_view.render(inner_area, buf),
            SelectedTab::Tab2 => self.logger.render(inner_area, buf),
            SelectedTab::Tab3 => self.controls.render(inner_area, buf),
        }

        if self.popup != PopupType::None {
            self.popup.render(area, buf);
        }
    }
}
