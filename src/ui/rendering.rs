use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Text};
use tui::widgets::{
    Axis, Block, Borders, Chart, Dataset, Gauge, GraphType, List, ListItem, ListState, Paragraph,
    Row, Table,
};
use tui::{symbols, Frame};

use crate::app::trainer::TrainerApp;
use crate::core::enums::AppState;
use crate::core::stats::TrainingRecord;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &TrainerApp)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Length(20), Constraint::Min(50)].as_ref())
        .split(f.size());
    draw_lesson_selection(f, app, chunks[0]);

    draw_lesson_details(f, app, chunks[1]);
}

fn draw_lesson_selection<B: Backend>(f: &mut Frame<B>, app: &TrainerApp, area: Rect) {
    let items: Vec<ListItem> = app
        .lessons()
        .iter()
        .map(|lesson| ListItem::new(Span::raw(lesson.name())))
        .collect();

    let lesson_selection = List::new(items)
        .block(Block::default().title("Lessons").borders(Borders::ALL))
        .style(get_style_depending_on_app_state(
            app.state(),
            AppState::LessonSelection,
        ))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");
    let mut selected_lesson_state = ListState::default();
    selected_lesson_state.select(app.lesson_list.selected_index());
    f.render_stateful_widget(lesson_selection, area, &mut selected_lesson_state)
}

fn draw_lesson_details<B: Backend>(f: &mut Frame<B>, app: &TrainerApp, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(area);
    draw_training(f, app, chunks[0]);
    draw_statistics(f, app, chunks[1]);
}

fn draw_training<B: Backend>(f: &mut Frame<B>, app: &TrainerApp, area: Rect) {
    let style = get_style_depending_on_app_state(app.state(), AppState::Training);
    let block = Block::default()
        .borders(Borders::ALL)
        .style(style)
        .title("Training");
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(4),
                Constraint::Min(9),
            ]
            .as_ref(),
        )
        .split(area);

    let progress_widget = Gauge::default()
        .block(Block::default().title("Progress:").borders(Borders::ALL))
        .gauge_style(style.add_modifier(Modifier::ITALIC | Modifier::BOLD))
        .ratio(app.lesson_progress.progress());
    f.render_widget(progress_widget, chunks[0]);

    let training_stats = app.lesson_progress.stats();
    let current_training_statistics_widget = Table::new(vec![
        // Row can be created from simple strings.
        Row::new(vec!["WPM", "Errors"]),
        Row::new(vec![
            training_stats.typing_speed.words_per_minute().to_string(),
            training_stats.errors.total_error_count.to_string(),
        ]),
    ])
    .widths(&[Constraint::Length(5), Constraint::Length(5)])
    .block(Block::default().title("Table").borders(Borders::ALL))
    .style(style);
    f.render_widget(current_training_statistics_widget, chunks[1]);

    let lesson_text_widget = Paragraph::new(app.lesson_progress.get_diff());
    f.render_widget(lesson_text_widget, chunks[2]);
}
fn draw_statistics<B: Backend>(f: &mut Frame<B>, app: &TrainerApp, area: Rect) {
    let block = Block::default().title("Statistics").borders(Borders::ALL);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .margin(1)
        .split(area);

    let current_training_records = app.lesson_list.current_lesson_records();
    let wpm_data = c![(x.0 as f64, x.1.stats.typing_speed.words_per_minute()as f64), for x in current_training_records.iter().enumerate()];

    let wpm_dataset = Dataset::default()
        .name("WPM")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Magenta))
        .data(&wpm_data);
    let wpm_chart = Chart::new(vec![wpm_dataset])
        .x_axis(
            Axis::default()
                .title(Span::styled("", Style::default()))
                .style(Style::default().fg(Color::White))
                .bounds([0.0, wpm_data.len() as f64]),
        )
        .y_axis(
            Axis::default()
                .title(Span::styled("WPM", Style::default().fg(Color::Red)))
                .style(Style::default().fg(Color::White))
                .bounds([0.0, 80.0])
                .labels(
                    ["0.0", "20", "40.0", "60.0", "80.0"]
                        .iter()
                        .cloned()
                        .map(Span::from)
                        .collect(),
                ),
        );

    f.render_widget(wpm_chart, chunks[0]);
    // let datasets = vec![
    //     Dataset::default()
    //         .name("Errors")
    //         .marker(symbols::Marker::Dot)
    //         .graph_type(GraphType::Line)
    //         .style(Style::default().fg(Color::Cyan))
    //         .data(&[(4.0, 5.0), (5.0, 8.0), (7.66, 13.5)]),
    //     Dataset::default()
    //         .name("WPM")
    //         .marker(symbols::Marker::Braille)
    //         .graph_type(GraphType::Line)
    //         .style(Style::default().fg(Color::Magenta))
    //         .data(&[(4.0, 5.0), (5.0, 8.0), (7.66, 13.5)]),
    // ];
    // let chart = Chart::new(datasets)
    //     .block(Block::default().title("Statistics").borders(Borders::ALL))
    //     .x_axis(
    //         Axis::default()
    //             .title(Span::styled("X Axis", Style::default().fg(Color::Red)))
    //             .style(Style::default().fg(Color::White))
    //             .bounds([0.0, 10.0])
    //             .labels(
    //                 ["0.0", "5.0", "10.0"]
    //                     .iter()
    //                     .cloned()
    //                     .map(Span::from)
    //                     .collect(),
    //             ),
    //     )
    //     .y_axis(
    //         Axis::default()
    //             .title(Span::styled("Y Axis", Style::default().fg(Color::Red)))
    //             .style(Style::default().fg(Color::White))
    //             .bounds([0.0, 10.0])
    //             .labels(
    //                 ["0.0", "5.0", "10.0"]
    //                     .iter()
    //                     .cloned()
    //                     .map(Span::from)
    //                     .collect(),
    //             ),
    //     );
    //
    // f.render_widget(chart, chunks[1]);
}
fn get_style_depending_on_app_state(current_state: &AppState, active_state: AppState) -> Style {
    if current_state == &active_state {
        get_active_style()
    } else {
        get_inactive_style()
    }
}

fn get_active_style() -> Style {
    Style::default().fg(Color::LightCyan)
}

fn get_inactive_style() -> Style {
    Style::default().fg(Color::DarkGray)
}
