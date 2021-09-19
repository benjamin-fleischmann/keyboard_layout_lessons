use crate::core::key::Key;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait ProvideKeyWeight {
    fn get_key_weight(&self, key: Key) -> f64;
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct FocusKey {
    pub focused_keys: Vec<Key>,
    pub weight: f64,
}

impl ProvideKeyWeight for FocusKey {
    fn get_key_weight(&self, key: Key) -> f64 {
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
    fn get_key_weight(&self, key: Key) -> f64 {
        match self {
            WeightingStrategy::EqualWeight => 1.0,
            WeightingStrategy::FocusKey(f) => f.get_key_weight(key),
        }
    }
}

#[cfg(test)]
mod test_weighting_strategy {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn always_eq_weight_always_returns_1() {
        let strategy = WeightingStrategy::EqualWeight;
        let current_key = Key { value: 'a' };
        assert_eq!(strategy.get_key_weight(current_key), 1.0)
    }

    #[test]
    fn focus_key_always_returns_1_for_non_focused_key() {
        let strategy = WeightingStrategy::FocusKey(FocusKey {
            focused_keys: vec![Key { value: 'a' }],
            weight: 10.0,
        });
        let current_key = Key { value: 'b' };
        assert_eq!(strategy.get_key_weight(current_key), 1.0)
    }

    #[test]
    fn focus_key_always_returns_weight_for_focused_key() {
        let strategy = WeightingStrategy::FocusKey(FocusKey {
            focused_keys: vec![Key { value: 'a' }],
            weight: 10.0,
        });
        let current_key = Key { value: 'a' };
        assert_eq!(strategy.get_key_weight(current_key), 10.0)
    }
}
