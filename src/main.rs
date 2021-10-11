use std::io;
use std::io::Stdout;
use std::time::Duration;

use termion::{event::Key, raw::IntoRawMode, raw::RawTerminal};
use tui::backend::TermionBackend;
use tui::Terminal;

use crate::app::trainer::TrainerApp;
use crate::core::enums::AppState;
use crate::ui::rendering::draw;

use self::core::enums::OptionalInput;
use crate::ui::events::Events;
#[macro_use]
extern crate cute;

mod app;
mod core;
mod ui;
mod wrapper;

fn start_terminal_gui(mut app: TrainerApp) -> Result<(), io::Error> {
    let mut terminal = create_initialized_terminal()?;
    // Setup event
    let events = Events::new(Duration::new(0, 500));
    loop {
        terminal.draw(|f| draw(f, &app))?;
        let event = events.next().unwrap_or(OptionalInput::InputKey(Key::Esc));

        app.tick(event);
        if app.state() == &AppState::Terminated {
            app.save();
            break;
        }
    }
    terminal.show_cursor()?;
    Ok(())
}

fn create_initialized_terminal() -> Result<Terminal<TermionBackend<RawTerminal<Stdout>>>, io::Error>
{
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    Ok(terminal)
}

fn main() -> anyhow::Result<()> {
    let mut app = TrainerApp::load(String::from("save.json"))?;
    start_terminal_gui(app)?;
    Ok(())
}
