use crate::lesson::Lesson;
use crate::key::Key;

pub fn create_home_row_lessons(lesson_length: u32, word_length: u8) -> Vec<Lesson>{
    let mut lessons: Vec<Lesson> = Vec::new();
    // home row "ctie ob nrsg q"
    lessons.push(Lesson::from_chars(&['i', 'e', 'n', 'r'],lesson_length,word_length));
    lessons.push(lessons.last().unwrap().add_key(Key::new('t')));
    lessons.push(lessons.last().unwrap().add_key(Key::new('s')));
    lessons.push(lessons.last().unwrap().add_key(Key::new('t')));
    lessons.push(lessons.last().unwrap().add_key(Key::new('s')));
    lessons.push(lessons.last().unwrap().add_key(Key::new('c')));
    lessons.push(lessons.last().unwrap().add_key(Key::new('g')));
    lessons.push(lessons.last().unwrap().add_key(Key::new('o')));
    lessons.push(lessons.last().unwrap().add_key(Key::new('b')));
    lessons.push(lessons.last().unwrap().add_key(Key::new('q')));
    lessons
}