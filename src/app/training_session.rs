use std::collections::VecDeque;
use std::iter::FromIterator;
use std::time::Duration;
#[cfg(not(test))]
use std::time::Instant;

#[cfg(test)]
use fake_clock::FakeClock as Instant;
use termion::event::Key;
use tui::style::{Color, Style};
use tui::text::{Span, Spans, Text};

enum InputResult {
    None,
    Wrong,
    Correct,
}
pub struct TrainingSession {
    pub lesson_content: String,
    finished_chars: Vec<char>,
    remaining_chars: VecDeque<char>,
    current_char: Option<char>,
    last_input_result: InputResult,
    start_time: Option<Instant>,
    end_time: Option<Instant>,
    errors: u16,
}
// struct LessonContent {
//     lesson_text: Vec<char>,
//     current_location: usize,
// }
//
// impl LessonContent {
//     pub fn new(content: String) -> Self {
//         Self{
//             lesson_text: Vec::from(content.chars()),
//             current_location: 0
//         }
//     }
//     pub fn finished_chars(&self) -> &[char] {
//         &self.lesson_text[..self.current_location]
//     }
//     pub fn remaining_chars(&self) -> &[char] {
//         &self.lesson_text[self.current_location..]
//     }
//     pub fn next_char(&self) -> Option<&char>{
//         self.lesson_text.get(self.current_location)
//     }
//     pub fn next(&mut)
// }
// #[cfg(test)]
// mod test_lesson_content {
//     use pretty_assertions::assert_eq;
//
//     use super::*;
//
//     #[test]
//     fn test_finished_chars() {
//         let strategy = WeightingStrategy::EqualWeight;
//         let current_key = Character { value: 'a' };
//         assert_eq!(strategy.get_key_weight(current_key), 1.0)
//     }
// }

impl TrainingSession {
    pub fn new(lesson: String) -> TrainingSession {
        let mut remaining_chars = VecDeque::from_iter(lesson.chars());
        let finished_chars: Vec<char> = Vec::with_capacity(remaining_chars.len());
        let next_char = remaining_chars.pop_front();
        TrainingSession {
            lesson_content: lesson,
            finished_chars,
            remaining_chars,
            current_char: next_char,
            last_input_result: InputResult::None,
            start_time: None,
            end_time: None,
            errors: 0,
        }
    }
    pub fn default() -> Self {
        TrainingSession::new(String::from(" "))
    }
    pub fn handle_key(&mut self, current_input: char) {
        if self.start_time == None {
            self.start_time = Some(Instant::now());
        }
        if current_input == self.current_char.unwrap() {
            self.finished_chars.push(current_input);
            self.current_char = self.remaining_chars.pop_front();
            self.last_input_result = InputResult::Correct;
        } else {
            self.errors += 1;
            self.last_input_result = InputResult::Wrong;
        }
        if self.is_finished() {
            self.end_time = Some(Instant::now());
        }
    }

    pub fn get_diff(&self) -> Text {
        const ZERO_WIDTH_CHAR: char = '\u{200B}';
        Text {
            lines: vec![Spans::from(vec![
                Span::styled(
                    String::from_iter(&self.finished_chars),
                    Style::default().fg(Color::Green),
                ),
                Span::styled(
                    self.current_char.unwrap_or(ZERO_WIDTH_CHAR).to_string(),
                    Style::default().fg(match self.last_input_result {
                        InputResult::None | InputResult::Correct => Color::Gray,
                        InputResult::Wrong => Color::Red,
                    }),
                ),
                Span::styled(
                    String::from_iter(&self.remaining_chars),
                    Style::default().fg(Color::DarkGray),
                ),
            ])],
        }
    }
    pub fn is_finished(&self) -> bool {
        self.current_char.is_none()
    }
    pub fn characters_per_minute(&self) -> Option<u16> {
        let duration: Duration = self.end_time.unwrap_or(Instant::now()) - self.start_time?;
        let characters_per_second: f32 = self.finished_chars.len() as f32 / duration.as_secs_f32();

        let characters_per_minute = characters_per_second * 60.0;
        Some(characters_per_minute as u16)
    }
    pub fn words_per_minute(&self) -> Option<u16> {
        Some(self.characters_per_minute()? / 5)
    }

    pub fn progress(&self) -> f64 {
        self.finished_chars.len() as f64 / self.lesson_content.len() as f64
    }
    pub fn errors(&self) -> u16 {
        self.errors
    }
}
#[cfg(test)]
mod test_training_session {
    use std::ops::Add;

    use pretty_assertions::assert_eq;

    use super::Instant as FakeClock;
    use super::*;

    #[test]
    fn test_characters_per_minute_also_counts_spaces() {
        let now = Instant::now();
        let in_1_minute = now.add(Duration::new(60, 0));
        let content_with_15_chars = "abcde fghijklmn";
        let unit = TrainingSession {
            lesson_content: content_with_15_chars.to_string(),
            finished_chars: Vec::from_iter(content_with_15_chars.chars()),
            remaining_chars: VecDeque::new(),
            current_char: None,
            last_input_result: InputResult::Correct,
            start_time: Some(now),
            end_time: Some(in_1_minute),
            errors: 0,
        };
        assert_eq!(unit.characters_per_minute(), Some(15))
    }
    #[test]
    fn test_characters_per_minute_is_undefined_if_not_started() {
        let now = Instant::now();
        let in_1_minute = now.add(Duration::new(60, 0));
        let content_with_15_chars = "abcde fghijklmn";
        let unit = TrainingSession {
            lesson_content: content_with_15_chars.to_string(),
            finished_chars: Vec::from_iter(content_with_15_chars.chars()),
            remaining_chars: VecDeque::new(),
            current_char: None,
            last_input_result: InputResult::Correct,
            start_time: None,
            end_time: None,
            errors: 0,
        };
        assert_eq!(unit.characters_per_minute(), None)
    }
    #[test]
    fn test_characters_per_minute_during_session() {
        let now = FakeClock::now();
        let one_minute_in_millis = 60 * 1000;
        FakeClock::advance_time(one_minute_in_millis);
        let content_with_15_chars = "abcde fghijklmn";
        let unit = TrainingSession {
            lesson_content: content_with_15_chars.to_string(),
            finished_chars: Vec::from_iter("abcde".chars()),
            remaining_chars: VecDeque::from_iter(" fghijklmn".chars()),
            current_char: None,
            last_input_result: InputResult::Correct,
            start_time: Some(now),
            end_time: None,
            errors: 0,
        };
        assert_eq!(unit.characters_per_minute(), Some(5))
    }
    #[test]
    fn test_words_per_minute_uses_5_char_per_word_convention() {
        let now = Instant::now();
        let in_1_minute = now.add(Duration::new(60, 0));
        let content_with_15_chars = "abcde fghijklmn";
        let unit = TrainingSession {
            lesson_content: content_with_15_chars.to_string(),
            finished_chars: Vec::from_iter(content_with_15_chars.chars()),
            remaining_chars: VecDeque::new(),
            current_char: None,
            last_input_result: InputResult::Correct,
            start_time: Some(now),
            end_time: Some(in_1_minute),
            errors: 0,
        };
        assert_eq!(
            unit.words_per_minute().unwrap() * 5,
            unit.characters_per_minute().unwrap()
        )
    }
}
