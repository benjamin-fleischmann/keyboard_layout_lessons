#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TypingSpeed {
    WordsPerMinute(u16),
    CharactersPerMinute(u16),
}

impl TypingSpeed {
    pub fn words_per_minute(self) -> u16 {
        match self {
            TypingSpeed::WordsPerMinute(wpm) => wpm,
            TypingSpeed::CharactersPerMinute(cpm) => cpm / 5,
        }
    }
}
#[cfg(test)]
mod test_training_session {
    use pretty_assertions::assert_eq;
    use std::ops::Add;

    use super::*;

    #[test]
    fn test_words_per_minute_uses_5_char_per_word_convention() {
        assert_eq!(TypingSpeed::CharactersPerMinute(15).words_per_minute(), 3)
    }
}
