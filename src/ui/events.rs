use std::io::stdin;
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::thread;
use std::time::Duration;

use termion::event::Event;
use termion::input::TermRead;

use crate::core::enums::OptionalInput;

pub struct Events {
    rx: Receiver<OptionalInput>,
    _tx: Sender<OptionalInput>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Events {
        let (tx, rx) = channel();

        let event_tx = tx.clone(); // the thread::spawn own event_tx
        thread::spawn(move || {
            for event_res in stdin().events() {
                match event_res {
                    Ok(event) => {
                        match event {
                            Event::Key(key) => {
                                // info!("event: {:?}", event);
                                event_tx.send(OptionalInput::InputKey(key)).unwrap();
                            }
                            _ => {
                                event_tx.send(OptionalInput::NoInput).unwrap();
                            }
                        }
                    }
                    Err(_) => {
                        event_tx.send(OptionalInput::NoInput).unwrap();
                        // error!("{:?}", err);
                    }
                }
            }
        });

        Events { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<OptionalInput, RecvError> {
        self.rx.recv()
    }
}
