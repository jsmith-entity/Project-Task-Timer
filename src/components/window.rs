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

use crate::{
    app::SessionState, config::KeyConfig, events::*, log_type::LogType, node::Node, traits::EventHandler,
};

use super::{Controls, LogView, PopupType, TaskView};

#[derive(Serialize, Deserialize, EnumIter, Display, Clone, Copy, PartialEq)]
enum SelectedTab {
    #[strum(to_string = "(1) Main")]
    Tab1,
    #[strum(to_string = "(2) Log")]
    Tab2,
    #[strum(to_string = "(3) Controls")]
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
    #[serde(skip)]
    key_config: KeyConfig,
}

impl Window {
    pub fn new(title: &str, key_config: KeyConfig) -> Self {
        Self {
            title: title.to_string(),

            task_view: TaskView::new(key_config),

            controls: Controls::new(),
            logger: LogView::new(key_config),

            selected_tab: SelectedTab::Tab1,
            popup: PopupType::None,
            key_config,
        }
    }

    pub fn load(&mut self, window: Window, key_config: KeyConfig) {
        self.title = window.title;

        self.task_view = TaskView::new_with(window.task_view, key_config);

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
        self.logger.update();
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

    pub async fn event(&mut self, key: KeyCode) -> anyhow::Result<EventState> {
        if self.popup.event(key).await?.is_consumed() {
            return Ok(EventState::Consumed);
        }

        if key == self.key_config.tab_main {
            self.selected_tab = SelectedTab::Tab1;
            return Ok(EventState::Consumed);
        }
        if key == self.key_config.tab_log {
            self.selected_tab = SelectedTab::Tab2;
            return Ok(EventState::Consumed);
        }
        if key == self.key_config.tab_controls {
            self.selected_tab = SelectedTab::Tab3;
            return Ok(EventState::Consumed);
        }

        match self.selected_tab {
            SelectedTab::Tab1 => {
                if self.task_view.event(key).await?.is_consumed() {
                    return Ok(EventState::Consumed);
                };
            }
            SelectedTab::Tab2 => {
                if self.logger.event(key).await?.is_consumed() {
                    return Ok(EventState::Consumed);
                }
            }
            _ => (),
        }

        return Ok(EventState::NotConsumed);

        // match res {
        //     Ok((log_type, info)) => {
        //         if log_type != InfoSubType::None {
        //             self.log(&log_type.message(info), LogType::INFO(log_type));
        //         }
        //     }
        //     Err(e) => self.log(&e, LogType::ERROR),
        // }
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
