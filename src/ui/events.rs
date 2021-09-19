use std::io::stdin;
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::thread;
use std::time::Duration;
use termion::event::{Event, Key};
use termion::input::TermRead;

pub enum OptionalKey {
    Key(Key),
    NoKey,
}

pub struct Events {
    rx: Receiver<OptionalKey>,
    _tx: Sender<OptionalKey>,
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
                                event_tx.send(OptionalKey::Key(key)).unwrap();
                            }
                            _ => {
                                event_tx.send(OptionalKey::NoKey).unwrap();
                            }
                        }
                    }
                    Err(_) => {
                        event_tx.send(OptionalKey::NoKey).unwrap();
                        // error!("{:?}", err);
                    }
                }
            }
        });

        Events { rx, _tx: tx }
    }

    /// Attempts to read an event.
    /// This function block the current thread.
    pub fn next(&self) -> Result<OptionalKey, RecvError> {
        self.rx.recv()
    }
}
