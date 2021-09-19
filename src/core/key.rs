use std::clone::Clone;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Key {
    pub value: char,
}

impl Key {
    pub fn new(value: char) -> Key {
        Key { value }
    }
}
