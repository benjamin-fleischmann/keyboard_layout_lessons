use std::clone::Clone;

use rand;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::core::character::Character;
use crate::core::weighting_strategy::WeightingStrategy;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Lesson {
    name: String,
    keys: Vec<Character>,
    weighting_strategy: WeightingStrategy,
    lesson_length: u32,
    word_length: u8,
}

impl Lesson {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn add_key(
        &self,
        name: String,
        key: Character,
        weighting_strategy: WeightingStrategy,
    ) -> Lesson {
        let mut new_keys = self.keys.clone();
        new_keys.push(key);
        Lesson {
            name,
            keys: new_keys,
            weighting_strategy,
            lesson_length: self.lesson_length,
            word_length: self.word_length,
        }
    }
    pub fn add_chars(
        &self,
        name: String,
        chars: &[char],
        weighting_strategy: WeightingStrategy,
    ) -> Lesson {
        let mut new_keys = self.keys.clone();
        for char in chars {
            new_keys.push(Character::new(*char))
        }
        Lesson {
            name,
            keys: new_keys,
            weighting_strategy,
            lesson_length: self.lesson_length,
            word_length: self.word_length,
        }
    }

    pub fn from_chars(
        name: String,
        chars: &[char],
        char_count: u32,
        word_length: u8,
        weighting_strategy: WeightingStrategy,
    ) -> Lesson {
        let mut new_keys: Vec<Character> = Vec::new();
        for char in chars {
            new_keys.push(Character::new(*char))
        }
        Lesson {
            name,
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
            let next_key = self.keys.choose(rng).unwrap().value;
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
            name: String::new(),
            keys: vec![Character { value: 'a' }],
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
            name: String::new(),
            keys: vec![Character { value: 'a' }],
            weighting_strategy: WeightingStrategy::EqualWeight,
            lesson_length: 10,
            word_length: 2,
        };
        let lesson_content = lesson.generate_lesson_content();
        assert_eq!(lesson_content, lesson_content.trim());
    }

    #[test]
    fn append_key_to_lesson() {
        let original_key = Character { value: 'a' };
        let original_lesson = Lesson {
            name: String::from("original_lesson"),
            keys: vec![original_key.clone()],
            weighting_strategy: WeightingStrategy::EqualWeight,
            lesson_length: 10,
            word_length: 2,
        };
        let extra_key = Character { value: 'b' };

        let expected_lesson = Lesson {
            name: String::from("lesson name"),
            keys: vec![original_key.clone(), extra_key.clone()],
            weighting_strategy: WeightingStrategy::EqualWeight,
            lesson_length: 10,
            word_length: 2,
        };
        let extended_lesson = original_lesson.add_key(
            String::from("lesson name"),
            extra_key,
            WeightingStrategy::EqualWeight,
        );

        assert_eq!(extended_lesson, expected_lesson)
    }
}
