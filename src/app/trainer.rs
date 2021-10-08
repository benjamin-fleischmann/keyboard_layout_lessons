use std::fs::OpenOptions;

use serde_json;
use termion::event::Key;

use crate::app::selectable_session_list::SelectableLessonList;
use crate::app::training_session::TrainingSession;
use crate::core::enums::{AppState, OptionalInput};
use crate::core::lesson::Lesson;

pub struct TrainerApp {
    pub lesson_list: SelectableLessonList,
    lesson_progress: TrainingSession,
    state: AppState,
    file_path: String,
}

impl TrainerApp {
    pub fn new(lessons: Vec<Lesson>) -> TrainerApp {
        TrainerApp {
            lesson_list: SelectableLessonList::new(lessons),
            lesson_progress: TrainingSession::default(),
            state: AppState::LessonSelection,
            file_path: String::new(),
        }
    }
    pub fn load(file_path: String) -> Result<TrainerApp, anyhow::Error> {
        let file = OpenOptions::new().read(true).open(&file_path)?;
        let data = serde_json::from_reader(file)?;
        Ok(TrainerApp {
            lesson_list: data,
            lesson_progress: TrainingSession::default(),
            state: AppState::LessonSelection,
            file_path,
        })
    }
    pub fn save(&self) {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)
            .unwrap();
        serde_json::to_writer(file, &self.lesson_list).unwrap();
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
                    self.lesson_list
                        .add_record_to_current_session(self.lesson_progress.training_record());
                    self.start_session();
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
