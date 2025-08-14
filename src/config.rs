use crossterm::event::KeyCode;

#[derive(Clone, Copy)]
pub struct KeyConfig {
    pub enter: KeyCode,
    pub quit: KeyCode,
    pub tab_main: KeyCode,
    pub tab_log: KeyCode,
    pub tab_controls: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub page_up: KeyCode,
    pub page_down: KeyCode,
    pub start_timer: KeyCode,
    pub complete: KeyCode,
    pub back: KeyCode,
    pub prev_subfilter: KeyCode,
    pub next_subfilter: KeyCode,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            enter: KeyCode::Enter,
            quit: KeyCode::Esc,
            tab_main: KeyCode::Char('1'),
            tab_log: KeyCode::Char('2'),
            tab_controls: KeyCode::Char('3'),
            up: KeyCode::Char('k'),
            down: KeyCode::Char('j'),
            left: KeyCode::Char('h'),
            right: KeyCode::Char('l'),
            page_up: KeyCode::Char('J'),
            page_down: KeyCode::Char('K'),
            start_timer: KeyCode::Char('s'),
            complete: KeyCode::Char(' '),
            back: KeyCode::Char('b'),
            prev_subfilter: KeyCode::Char('H'),
            next_subfilter: KeyCode::Char('L'),
        }
    }
}
