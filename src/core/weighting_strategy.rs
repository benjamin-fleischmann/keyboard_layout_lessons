use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::core::character::Character;

pub trait ProvideKeyWeight {
    fn get_key_weight(&self, key: Character) -> f64;
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct FocusKey {
    pub focused_keys: Vec<Character>,
    pub weight: f64,
}

impl ProvideKeyWeight for FocusKey {
    fn get_key_weight(&self, key: Character) -> f64 {
        if self.focused_keys.contains(&key) {
            self.weight
        } else {
            1.0
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum WeightingStrategy {
    EqualWeight,
    FocusKey(FocusKey),
}

impl ProvideKeyWeight for WeightingStrategy {
    fn get_key_weight(&self, key: Character) -> f64 {
        match self {
            WeightingStrategy::EqualWeight => 1.0,
            WeightingStrategy::FocusKey(f) => f.get_key_weight(key),
        }
    }
}

#[cfg(test)]
mod test_weighting_strategy {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn always_eq_weight_always_returns_1() {
        let strategy = WeightingStrategy::EqualWeight;
        let current_key = Character { value: 'a' };
        assert_eq!(strategy.get_key_weight(current_key), 1.0)
    }

    #[test]
    fn focus_key_always_returns_1_for_non_focused_key() {
        let strategy = WeightingStrategy::FocusKey(FocusKey {
            focused_keys: vec![Character { value: 'a' }],
            weight: 10.0,
        });
        let current_key = Character { value: 'b' };
        assert_eq!(strategy.get_key_weight(current_key), 1.0)
    }

    #[test]
    fn focus_key_always_returns_weight_for_focused_key() {
        let strategy = WeightingStrategy::FocusKey(FocusKey {
            focused_keys: vec![Character { value: 'a' }],
            weight: 10.0,
        });
        let current_key = Character { value: 'a' };
        assert_eq!(strategy.get_key_weight(current_key), 10.0)
    }
}
