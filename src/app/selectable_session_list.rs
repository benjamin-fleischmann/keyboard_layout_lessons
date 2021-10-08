use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::core::lesson::Lesson;
use crate::core::stats::TrainingRecord;

#[derive(Serialize, Deserialize)]
pub struct SelectableLessonList {
    lessons: Vec<Lesson>,
    selected_index: Option<usize>,
    training_records: HashMap<usize, Vec<TrainingRecord>>,
}

impl SelectableLessonList {
    pub fn new(lessons: Vec<Lesson>) -> Self {
        Self {
            lessons,
            selected_index: None,
            training_records: HashMap::new(),
        }
    }
    pub fn lessons(&self) -> &[Lesson] {
        &self.lessons[..]
    }
    pub fn selected_index(&self) -> Option<usize> {
        self.selected_index
    }
    pub fn current_lesson(&self) -> Option<&Lesson> {
        self.lessons.get(self.selected_index?)
    }

    pub fn current_lesson_records(&self) -> &[TrainingRecord] {
        //Option<&Vec<TrainingRecord>> {
        let maybe_data = match self.selected_index {
            None => None,
            Some(index) => self.training_records.get(&index),
        };
        match maybe_data {
            Some(a) => a.as_ref(),
            None => &[] as &[TrainingRecord],
        }
    }
    pub fn select_next_lesson(&mut self) {
        match self.selected_index {
            None => {
                if !self.lessons.is_empty() {
                    self.selected_index = Some(0);
                }
            }
            Some(current) => {
                let next = current + 1;
                if self.lessons.len() > next {
                    self.selected_index = Some(next);
                }
            }
        }
    }
    pub fn select_prev_lesson(&mut self) {
        match self.selected_index {
            None => {
                if !self.lessons.is_empty() {
                    self.selected_index = Some(self.lessons.len() - 1);
                }
            }
            Some(0) => {}
            Some(current) => {
                let next = current - 1;
                self.selected_index = Some(next);
            }
        }
    }
    pub fn add_record_to_current_session(&mut self, trainig_record: TrainingRecord) {
        let entry = self
            .training_records
            .entry(self.selected_index.unwrap())
            .or_insert(Vec::new());
        entry.push(trainig_record);
    }
}
#[cfg(test)]
mod test_selectable_session_list {
    use pretty_assertions::assert_eq;

    use crate::core::weighting_strategy::WeightingStrategy;

    use super::*;

    fn get_sample_lessons() -> Vec<Lesson> {
        let lesson_1 = Lesson::from_chars(
            String::from("Lesson 1"),
            &['1'],
            10,
            4,
            WeightingStrategy::EqualWeight,
        );
        let lesson_2 = lesson_1.add_chars(
            String::from("Lesson 2"),
            &['2'],
            WeightingStrategy::EqualWeight,
        );
        vec![lesson_1, lesson_2]
    }
    #[test]
    fn test_select_next_lesson() {
        let mut unit = SelectableLessonList {
            lessons: get_sample_lessons(),
            selected_index: Some(0),
            training_records: HashMap::new(),
        };
        unit.select_next_lesson();
        assert_eq!(unit.selected_index, Some(1))
    }
    #[test]
    fn test_select_next_lesson_when_last_index_is_selected() {
        let mut unit = SelectableLessonList {
            lessons: get_sample_lessons(),
            selected_index: Some(1),
            training_records: HashMap::new(),
        };
        unit.select_next_lesson();
        assert_eq!(unit.selected_index, Some(1))
    }
    #[test]
    fn test_select_next_lesson_when_none_is_selected() {
        let mut unit = SelectableLessonList {
            lessons: get_sample_lessons(),
            selected_index: None,
            training_records: HashMap::new(),
        };
        unit.select_next_lesson();
        assert_eq!(unit.selected_index, Some(0))
    }
    #[test]
    fn test_select_prev_lesson() {
        let mut unit = SelectableLessonList {
            lessons: get_sample_lessons(),
            selected_index: Some(1),
            training_records: HashMap::new(),
        };
        unit.select_prev_lesson();
        assert_eq!(unit.selected_index, Some(0))
    }
    #[test]
    fn test_select_prev_lesson_when_first_index_is_selected() {
        let mut unit = SelectableLessonList {
            lessons: get_sample_lessons(),
            selected_index: Some(0),
            training_records: HashMap::new(),
        };
        unit.select_prev_lesson();
        assert_eq!(unit.selected_index, Some(0))
    }
    #[test]
    fn test_select_prev_lesson_when_none_is_selected() {
        let mut unit = SelectableLessonList {
            lessons: get_sample_lessons(),
            selected_index: None,
            training_records: HashMap::new(),
        };
        unit.select_prev_lesson();
        assert_eq!(unit.selected_index, Some(1))
    }
    #[test]
    fn test_get_current_lesson() {
        let unit = SelectableLessonList {
            lessons: get_sample_lessons(),
            selected_index: Some(0),
            training_records: HashMap::new(),
        };
        assert_eq!(unit.current_lesson(), unit.lessons.get(0))
    }
    #[test]
    fn test_get_current_lesson_when_none_is_selected() {
        let unit = SelectableLessonList {
            lessons: get_sample_lessons(),
            selected_index: None,
            training_records: HashMap::new(),
        };
        assert_eq!(unit.current_lesson(), None)
    }
}
