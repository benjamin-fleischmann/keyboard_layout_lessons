use keyboard_layout_lessons_r::lesson::Lesson;
use keyboard_layout_lessons_r::key::Key;
use keyboard_layout_lessons_r::bone;
use keyboard_layout_lessons_r::trainer;

fn main() {
    let lessons = bone::create_home_row_lessons(200,4);
    let lesson_trainer = trainer::Trainer(lessons.get(0).unwrap());
    lesson_trainer.train();
}