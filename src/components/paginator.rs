use ratatui::{
    prelude::{Buffer, Rect},
    text::Line,
    widgets::Widget,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Paginator {
    pub page: usize,
    pub page_size: usize,
    pub entry_len: usize,
}

impl Paginator {
    pub fn next_page(&mut self) {
        if (self.page + 1) * self.page_size < self.entry_len {
            self.page += 1;
        }
    }

    pub fn prev_page(&mut self) {
        self.page = self.page.saturating_sub(1);
    }

    pub fn offset(&self) -> usize {
        return self.page * self.page_size;
    }

    pub fn content_height(&self) -> u16 {
        let height: u16;

        let max_page_len = self.offset() + self.page_size;
        if max_page_len > self.entry_len {
            height = (self.entry_len - self.offset()) as u16;
        } else {
            height = max_page_len as u16;
        }

        return height;
    }

    pub fn page_slice(&self) -> (usize, usize) {
        let total_logs = self.entry_len;
        let start_idx = self.page * self.page_size;
        let end_idx = std::cmp::min(start_idx + self.page_size, total_logs);

        return (start_idx, end_idx);
    }
}

impl Widget for &Paginator {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let total_pages = self.entry_len / self.page_size + 1;
        let text = format!("↑↓ Page: ({}/{})", self.page + 1, total_pages);
        Line::from(text).render(area, buf);
    }
}
