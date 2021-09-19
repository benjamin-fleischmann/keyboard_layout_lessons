use std::collections::VecDeque;
use std::iter::FromIterator;
use std::str::Chars;

use text_diff::{diff, Difference};
use tui::style::{Color, Style};
use tui::text::{Span, Spans, Text};

use crate::core::lesson::Lesson;
use std::alloc::Layout;

// pub struct TrainerApp<'a> {}
pub struct App<'a> {
    pub lesson: &'a str,
    finished_chars: Vec<char>,
    remaining_chars: VecDeque<char>,
    next_char: Option<char>,
    last_input: Option<char>,
}

impl App<'_> {
    pub fn new(lesson: &str) -> App {
        let mut remaining_chars = VecDeque::from_iter(lesson.chars());
        let finished_chars: Vec<char> = Vec::with_capacity(remaining_chars.len());
        let next_char = remaining_chars.pop_front();
        App {
            lesson,
            finished_chars,
            remaining_chars,
            next_char,
            last_input: None,
        }
    }

    pub fn add_key(&mut self, current_input: char) {
        if current_input == self.next_char.unwrap() {
            self.finished_chars.push(current_input);
            self.next_char = self.remaining_chars.pop_front();
        }
        self.last_input = Some(current_input)
    }
    pub fn get_diff(&self) -> Text {
        Text {
            lines: vec![Spans::from(vec![
                Span::styled(
                    String::from_iter(&self.finished_chars),
                    Style::default().fg(Color::Green),
                ),
                Span::styled(
                    self.next_char.unwrap_or('\u{200B}').to_string(),
                    Style::default().fg(Color::Red),
                ),
                Span::styled(
                    String::from_iter(&self.remaining_chars),
                    Style::default().fg(Color::Gray),
                ),
            ])],
        }
    }
    pub fn is_finished(&self) -> bool {
        self.next_char.is_none()
    }
}
