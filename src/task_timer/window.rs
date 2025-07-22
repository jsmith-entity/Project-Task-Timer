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
    node::Node,
    popup::Popup,
    views::{
        controls::*,
        home::main_view::*,
        log::{log_type::*, log_view::*},
    },
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

    main_view: MainView,
    logger: LogView,
    #[serde(skip)]
    controls: Controls,
    #[serde(skip)]
    popup: Option<Popup>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            title: "???".to_string(),

            main_view: MainView::new(),

            controls: Controls::new(),
            logger: LogView::new(),

            selected_tab: SelectedTab::Tab1,
            popup: None,
        }
    }

    pub fn load(&mut self, window: Window) {
        self.title = window.title;

        self.main_view = MainView::new_with(window.main_view);

        self.controls = Controls::new();
        self.logger = window.logger;

        self.selected_tab = window.selected_tab;
        self.popup = None;
    }
}

impl Window {
    pub fn update_tree(&mut self, new_root: Node) {
        self.main_view.root_node = new_root.clone();
        self.main_view.update_display_data(new_root);
    }

    pub fn update_time(&mut self) {
        if let Err(e) = self.main_view.update_time() {
            self.log(&e, LogType::ERROR);
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
            let res = match self.selected_tab {
                SelectedTab::Tab1 => self.main_view.handle_events(key_code),
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
    }

    pub fn log(&mut self, message: &str, log_type: LogType) {
        self.logger.log(message, log_type);
    }

    pub fn enable_popup(&mut self, message: &str) {
        let new_popup = Popup::new(message.to_string());
        self.popup = Some(new_popup);
    }

    pub fn disable_popup(&mut self) {
        self.popup = None;
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
            SelectedTab::Tab1 => self.main_view.render(inner_area, buf),
            SelectedTab::Tab2 => self.logger.render(inner_area, buf),
            SelectedTab::Tab3 => self.controls.render(inner_area, buf),
        }

        if self.popup.is_some() {
            self.popup.as_ref().unwrap().render(area, buf);
        }
    }
}
