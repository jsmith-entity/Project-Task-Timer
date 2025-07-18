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
    node::Node,
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

#[derive(Serialize, Deserialize)]
pub struct Window {
    pub title: String,

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
        self.main_view.root_node = new_root.clone();
        if let Err(e) = self.main_view.update_display_data(new_root) {
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
                //SelectedTab::Tab2 => self.handle_log_events(key_code),
                _ => Ok(()),
            };

            if let Err(e) = res {
                self.log(&e, LogType::ERROR);
            }
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

    fn handle_log_events(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('h') => self.log.prev_filter(),
            KeyCode::Char('l') => self.log.next_filter(),
            _ => (),
        }
    }
}
