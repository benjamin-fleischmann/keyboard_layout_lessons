use crate::app::trainer::TrainerApp;
use crate::core::lesson::Lesson;
use crate::core::weighting_strategy::WeightingStrategy;

fn create_bone_home_row_lessons() -> Vec<Lesson> {
    let lesson_length = 8;
    let word_length = 4;

    let mut lessons: Vec<Lesson> = Vec::new();
    // home row "ctie ob nrsg q"
    let lesson_1 = Lesson::from_chars(
        String::from("Lesson 1"),
        &['i', 'e', 'n', 'r'],
        lesson_length,
        word_length,
        WeightingStrategy::EqualWeight,
    );

    let lesson_2 = lesson_1.add_chars(
        String::from("Lesson 2"),
        &['t', 's'],
        WeightingStrategy::EqualWeight,
    );

    let lesson_3 = lesson_2.add_chars(
        String::from("Lesson 3"),
        &['c', 'g'],
        WeightingStrategy::EqualWeight,
    );

    let lesson_4 = lesson_3.add_chars(
        String::from("Lesson 4"),
        &['o', 'b'],
        WeightingStrategy::EqualWeight,
    );

    let lesson_5 = lesson_4.add_chars(
        String::from("Lesson 5"),
        &['q'],
        WeightingStrategy::EqualWeight,
    );
    lessons.push(lesson_1);
    lessons.push(lesson_2);
    lessons.push(lesson_3);
    lessons.push(lesson_4);
    lessons.push(lesson_5);

    lessons
}
pub fn create_bone_trainer() -> TrainerApp {
    let home_row_lessons = self::create_bone_home_row_lessons();
    TrainerApp::new(home_row_lessons)
}
