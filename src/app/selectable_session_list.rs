use crate::core::lesson::Lesson;

pub struct SelectableLessonList {
    lessons: Vec<Lesson>,
    selected_index: Option<usize>,
}
impl SelectableLessonList {
    pub fn new(lessons: Vec<Lesson>) -> Self {
        Self {
            lessons,
            selected_index: None,
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
        };
        unit.select_next_lesson();
        assert_eq!(unit.selected_index, Some(1))
    }
    #[test]
    fn test_select_next_lesson_when_last_index_is_selected() {
        let mut unit = SelectableLessonList {
            lessons: get_sample_lessons(),
            selected_index: Some(1),
        };
        unit.select_next_lesson();
        assert_eq!(unit.selected_index, Some(1))
    }
    #[test]
    fn test_select_next_lesson_when_none_is_selected() {
        let mut unit = SelectableLessonList {
            lessons: get_sample_lessons(),
            selected_index: None,
        };
        unit.select_next_lesson();
        assert_eq!(unit.selected_index, Some(0))
    }
    #[test]
    fn test_select_prev_lesson() {
        let mut unit = SelectableLessonList {
            lessons: get_sample_lessons(),
            selected_index: Some(1),
        };
        unit.select_prev_lesson();
        assert_eq!(unit.selected_index, Some(0))
    }
    #[test]
    fn test_select_prev_lesson_when_first_index_is_selected() {
        let mut unit = SelectableLessonList {
            lessons: get_sample_lessons(),
            selected_index: Some(0),
        };
        unit.select_prev_lesson();
        assert_eq!(unit.selected_index, Some(0))
    }
    #[test]
    fn test_select_prev_lesson_when_none_is_selected() {
        let mut unit = SelectableLessonList {
            lessons: get_sample_lessons(),
            selected_index: None,
        };
        unit.select_prev_lesson();
        assert_eq!(unit.selected_index, Some(1))
    }
    #[test]
    fn test_get_current_lesson() {
        let mut unit = SelectableLessonList {
            lessons: get_sample_lessons(),
            selected_index: Some(0),
        };
        assert_eq!(unit.current_lesson(), unit.lessons.get(0))
    }
    #[test]
    fn test_get_current_lesson_when_none_is_selected() {
        let mut unit = SelectableLessonList {
            lessons: get_sample_lessons(),
            selected_index: None,
        };
        assert_eq!(unit.current_lesson(), None)
    }
}
