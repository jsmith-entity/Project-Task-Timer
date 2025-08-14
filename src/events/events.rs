use crossterm::{event, event::KeyEvent};

use std::{sync::mpsc, thread, time::Duration};

pub enum InputEvent {
    Input(KeyEvent),
    Tick,
}

pub struct Events {
    rx: mpsc::Receiver<InputEvent>,
    _tx: mpsc::Sender<InputEvent>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();

        let event_tx = tx.clone();
        thread::spawn(move || {
            loop {
                if crossterm::event::poll(tick_rate).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        event_tx.send(InputEvent::Input(key)).unwrap();
                    }
                }
                event_tx.send(InputEvent::Tick).unwrap();
            }
        });

        return Self { rx, _tx: tx };
    }

    pub fn next(&self) -> Result<InputEvent, mpsc::RecvError> {
        return self.rx.recv();
    }
}

#[derive(PartialEq, Eq)]
pub enum EventState {
    Consumed,
    NotConsumed,
}

impl EventState {
    pub fn is_consumed(&self) -> bool {
        return *self == Self::Consumed;
    }
}
