#[derive(PartialEq, Debug)]
pub struct Key {
    pub value: char,
    pub weight: f64,
}

impl Clone for Key {
    fn clone(&self) -> Self {
        Key {
            value: self.value,
            weight: self.weight,
        }
    }
}

impl Key {
    pub fn new(value: char) -> Key {
        Key { value, weight: 1.0 }
    }
}