use std::clone::Clone;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    pub value: char,
}

impl Character {
    pub fn new(value: char) -> Character {
        Character { value }
    }
}
