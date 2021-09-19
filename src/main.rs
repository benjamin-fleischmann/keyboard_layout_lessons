use std::cell::RefCell;
use std::io;
use std::io::Stdout;
use std::rc::Rc;
use std::sync::mpsc::RecvError;
use std::time::Duration;

use termion::{
    event::Key, input::MouseTerminal, raw::IntoRawMode, raw::RawTerminal, screen::AlternateScreen,
};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};
use tui::Terminal;

use crate::ui::app::App;
use crate::ui::events::{Events, OptionalKey};
use crate::ui::rendering::draw;

mod core;
mod layouts;
mod ui;

fn start_terminal_gui() -> Result<(), io::Error> {
    //app: Rc<RefCell<App>>
    // Terminal initialization
    let mut terminal = initialize_terminal()?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    // Setup event
    let events = Events::new(Duration::new(0, 500));
    let mut app = App::new("astasdasr tdsrdasd");
    loop {
        terminal.draw(|mut f| draw(f, &app));
        let event = events.next().unwrap_or(OptionalKey::Key(Key::Esc));
        match event {
            OptionalKey::Key(key) => match key {
                Key::Backspace => {}
                Key::Left => {}
                Key::Right => {}
                Key::Up => {}
                Key::Down => {}
                Key::Home => {}
                Key::End => {}
                Key::PageUp => {}
                Key::PageDown => {}
                Key::BackTab => {}
                Key::Delete => {}
                Key::Insert => {}
                Key::F(_) => {}
                Key::Char(c) => app.add_key(c),
                Key::Alt(_) => {}
                Key::Ctrl(_) => {}
                Key::Null => {}
                Key::Esc => {
                    break;
                }
                Key::__IsNotComplete => {}
            },
            OptionalKey::NoKey => {
                continue;
            }
        }
        if app.is_finished() {
            break;
        };
    }

    terminal.show_cursor()?;
    Ok(())
}

fn initialize_terminal() -> Result<Terminal<TermionBackend<RawTerminal<Stdout>>>, io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    Terminal::new(backend)
}

fn main() -> Result<(), io::Error> {
    // let app = Rc::new(RefCell::new(App::new())); // TODO app is useless for now
    start_terminal_gui()?;
    Ok(())
}
// fn main() {
// let lessons = bone::create_home_row_lessons(200,4);
// let lesson_trainer = trainer::Trainer(lessons.get(0).unwrap());
// lesson_trainer.train();
// }
