use rand;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::Ordering::Less;

use crate::core::key::Key;
use crate::core::weighting_strategy::WeightingStrategy;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Lesson {
    pub keys: Vec<Key>,
    pub weighting_strategy: WeightingStrategy,
    pub lesson_length: u32,
    pub word_length: u8,
}

impl Lesson {
    pub fn add_key(&self, key: Key, weighting_strategy: WeightingStrategy) -> Lesson {
        let mut new_keys = self.keys.clone();
        new_keys.push(key);
        Lesson {
            keys: new_keys,
            weighting_strategy,
            lesson_length: self.lesson_length,
            word_length: self.word_length,
        }
    }
    pub fn from_chars(
        chars: &[char],
        char_count: u32,
        word_length: u8,
        weighting_strategy: WeightingStrategy,
    ) -> Lesson {
        let mut new_keys: Vec<Key> = Vec::new();
        for char in chars {
            new_keys.push(Key::new(*char))
        }
        Lesson {
            keys: new_keys,
            weighting_strategy,
            lesson_length: char_count,
            word_length,
        }
    }
    fn generate_word(&self) -> String {
        let mut word = String::new();
        let rng = &mut rand::thread_rng();
        while word.len() < self.word_length as usize {
            let next_key = self.keys.choose_weighted(rng, |item| 1.0).unwrap().value;
            word.push(next_key);
        }
        word
    }
    pub fn generate_lesson_content(&self) -> String {
        let mut content = self.generate_word();
        while content.len() < self.lesson_length as usize {
            content = [content, self.generate_word()].join(" ");
        }
        content
    }
}

#[cfg(test)]
mod test_lesson {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn has_roughly_specified_length() {
        let lesson = Lesson {
            keys: vec![Key { value: 'a' }],
            weighting_strategy: WeightingStrategy::EqualWeight,
            lesson_length: 10,
            word_length: 2,
        };
        let lesson_length = lesson.generate_lesson_content().len() as u32;
        assert!(
            lesson.lesson_length <= lesson_length
                && lesson_length <= lesson.lesson_length + lesson.word_length as u32
        )
    }

    #[test]
    fn content_does_not_start_or_end_with_whitespace() {
        let lesson = Lesson {
            keys: vec![Key { value: 'a' }],
            weighting_strategy: WeightingStrategy::EqualWeight,
            lesson_length: 10,
            word_length: 2,
        };
        let lesson_content = lesson.generate_lesson_content();
        assert_eq!(lesson_content, lesson_content.trim());
    }

    #[test]
    fn append_key_to_lesson() {
        let original_key = Key { value: 'a' };
        let original_lesson = Lesson {
            keys: vec![original_key.clone()],
            weighting_strategy: WeightingStrategy::EqualWeight,
            lesson_length: 10,
            word_length: 2,
        };
        let extra_key = Key { value: 'b' };

        let expected_lesson = Lesson {
            keys: vec![original_key.clone(), extra_key.clone()],
            weighting_strategy: WeightingStrategy::EqualWeight,
            lesson_length: 10,
            word_length: 2,
        };
        let extended_lesson = original_lesson.add_key(extra_key, WeightingStrategy::EqualWeight);

        assert_eq!(extended_lesson, expected_lesson)
    }
}
