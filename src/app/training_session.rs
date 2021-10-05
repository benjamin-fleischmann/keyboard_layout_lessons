#[cfg(not(test))]
use crate::wrapper::clock::Clock;
#[cfg(test)]
use crate::wrapper::fake_clock::FakeClock as Clock;

use std::collections::VecDeque;
use std::iter::FromIterator;

use chrono::Duration;
use chrono::{DateTime, Utc};
// use std::time::Duration;

// use std::time::SystemTime;
use termion::event::Key;
use tui::style::{Color, Style};
use tui::text::{Span, Spans, Text};

use crate::core::stats::{TrainingRecord, TrainingStatistics};
use crate::core::typing_errors::TypingErrors;
use crate::core::typing_speed::TypingSpeed;

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
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    errors: u16,
}

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
            self.start_time = Some(Clock::now());
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
            self.end_time = Some(Clock::now());
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
    pub fn typing_speed(&self) -> TypingSpeed {
        if let Some(start_time) = self.start_time {
            let duration: Duration = self.end_time.unwrap_or(Clock::now()) - start_time;
            if duration < Duration::seconds(1) {
                TypingSpeed::CharactersPerMinute(0)
            } else {
                let characters_per_minute =
                    60 * self.finished_chars.len() as u32 / duration.num_seconds() as u32;
                TypingSpeed::CharactersPerMinute(characters_per_minute as u16)
            }
        } else {
            TypingSpeed::CharactersPerMinute(0)
        }
    }

    pub fn progress(&self) -> f64 {
        self.finished_chars.len() as f64 / self.lesson_content.len() as f64
    }

    pub fn errors(&self) -> TypingErrors {
        TypingErrors {
            total_error_count: self.errors,
        }
    }
    pub fn stats(&self) -> TrainingStatistics {
        TrainingStatistics {
            errors: self.errors(),
            typing_speed: self.typing_speed(),
        }
    }
    pub fn training_record(&self) -> TrainingRecord {
        TrainingRecord {
            timestamp: self.start_time.unwrap_or(Utc::now()),
            stats: self.stats(),
        }
    }
}
#[cfg(test)]
mod test_training_session {
    use std::ops::Add;

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::wrapper::fake_clock::FakeClock;

    #[test]
    fn test_characters_per_minute_also_counts_spaces() {
        let now = Clock::now();
        let in_1_minute = now.add(Duration::minutes(1));
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
        assert_eq!(unit.typing_speed(), TypingSpeed::CharactersPerMinute(15))
    }
    #[test]
    fn test_characters_per_minute_is_undefined_if_not_started() {
        let now = Utc::now();
        let in_1_minute = now.add(Duration::minutes(1));
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
        assert_eq!(unit.typing_speed(), TypingSpeed::CharactersPerMinute(0))
    }
    #[test]
    fn test_characters_per_minute_during_session() {
        let now = FakeClock::now();
        FakeClock::advance(Duration::minutes(1));
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
        assert_eq!(unit.typing_speed(), TypingSpeed::CharactersPerMinute(5))
    }
}
