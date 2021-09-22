use termion::event::Key;

pub enum OptionalInput {
    InputKey(Key),
    NoInput,
}

#[derive(PartialEq)]
pub enum AppState {
    LessonSelection,
    Training,
    Terminated,
}
