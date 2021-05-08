use crate::lesson::Lesson;
use termion::color;

pub struct Trainer<'a>{
    lesson: &'a Lesson
}

impl Trainer<'_>{
    pub fn train(&self)  {
        let lesson_content = self.lesson.generate_lesson_content();
        println!("{}", color::Fg(color::LightWhite));

    }
}

#[cfg(test)]
mod test_lesson {
    use super::*;
 }
