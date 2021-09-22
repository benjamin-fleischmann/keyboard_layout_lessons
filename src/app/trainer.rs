use termion::event::Key;

use crate::app::selectable_session_list::SelectableLessonList;
use crate::app::training_session::TrainingSession;
use crate::core::enums::{AppState, OptionalInput};
use crate::core::lesson::Lesson;

pub struct TrainerApp {
    pub lesson_list: SelectableLessonList,
    pub lesson_progress: TrainingSession,
    state: AppState,
}

impl TrainerApp {
    pub fn new(lessons: Vec<Lesson>) -> TrainerApp {
        TrainerApp {
            lesson_list: SelectableLessonList::new(lessons),
            lesson_progress: TrainingSession::default(),
            state: AppState::LessonSelection,
        }
    }
    pub fn state(&self) -> &AppState {
        &self.state
    }
    pub fn lessons(&self) -> &[Lesson] {
        &self.lesson_list.lessons()
    }
    pub fn lesson_progress(&self) -> &TrainingSession {
        &self.lesson_progress
    }

    pub fn tick(&mut self, optional_input: OptionalInput) {
        match optional_input {
            OptionalInput::InputKey(key) => self.handle_input(key),
            OptionalInput::NoInput => {}
        }
    }
    fn handle_input(&mut self, input_key: Key) {
        match self.state {
            AppState::Training => self.handle_training(input_key),
            AppState::LessonSelection => self.handle_lesson_selection(input_key),
            AppState::Terminated => {}
        }
    }

    fn handle_training(&mut self, input_key: Key) {
        match input_key {
            Key::Esc => {
                self.state = AppState::LessonSelection;
            }
            Key::Char(c) => {
                self.lesson_progress.handle_key(c);
                if self.lesson_progress.is_finished() {
                    self.state = AppState::LessonSelection;
                }
            }
            _ => {}
        }
    }

    fn handle_lesson_selection(&mut self, input_key: Key) {
        match input_key {
            Key::Esc => {
                self.state = AppState::Terminated;
            }
            Key::Down => {
                self.lesson_list.select_next_lesson();
            }
            Key::Up => {
                self.lesson_list.select_prev_lesson();
            }
            Key::Char('\n') => {
                self.start_session();
            }
            _ => {}
        }
    }

    pub fn start_session(&mut self) {
        if let Some(lesson) = self.lesson_list.current_lesson() {
            self.lesson_progress = TrainingSession::new(lesson.generate_lesson_content());
            self.state = AppState::Training;
        }
    }
}

#[cfg(test)]
mod test_lesson {}
