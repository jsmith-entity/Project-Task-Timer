use crossterm::event::KeyCode;

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
        }
    }
}
