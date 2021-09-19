use std::clone::Clone;
use std::io;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;

use crate::ui::app::App;
use tui::text::Text;

//
// // #[derive(Error, Debug)]
// // pub enum Error {
// //     #[error("error reading the DB file: {0}")]
// //     ReadDBError(#[from] io::Error),
// //     #[error("error parsing the DB file: {0}")]
// //     ParseDBError(#[from] serde_json::Error),
// // }
// enum Event<I> {
//     Input(I),
//     Tick,
// }
//
// #[derive(Copy, Clone, Debug)]
// enum MenuItem {
//     Home,
//     Lessons,
// }
//
// impl From<MenuItem> for usize {
//     fn from(input: MenuItem) -> usize {
//         match input {
//             MenuItem::Home => 0,
//             MenuItem::Lessons => 1,
//         }
//     }
// }
//
pub fn draw<B>(f: &mut Frame<B>, _app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let lesson_overview =
        Paragraph::new(_app.lesson).block(Block::default().title("Lesson:").borders(Borders::ALL));
    f.render_widget(lesson_overview, chunks[0]);

    let progress_block = Paragraph::new(_app.get_diff())
        .block(Block::default().title("Progress:").borders(Borders::ALL));
    f.render_widget(progress_block, chunks[1]);
}
