use std::fs::File;
use std::io::Write;

use keyboard_layout_lessons_r::lesson::Lesson;
use keyboard_layout_lessons_r::key::Key;
use keyboard_layout_lessons_r::bone;

fn write_lesson_content_to_file(lesson: &Lesson, number: u8) -> () {
    let lesson_content = lesson.generate_lesson_content();
    let output_path = format!("output/lesson{}.txt", number);
    let mut output = File::create(output_path).expect("failed to create file");
    write!(output, "{}", &lesson_content).expect("failed to write content");
}

fn main() {
    let lessons: Vec<Lesson> = bone::create_home_row_lessons(500, 4);
    for (number, lesson) in lessons.iter().enumerate() {
        write_lesson_content_to_file(lesson, number as u8);
    }
}
