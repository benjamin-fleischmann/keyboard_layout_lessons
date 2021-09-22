use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Text};
use tui::widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph, Row, Table};
use tui::Frame;

use crate::app::trainer::TrainerApp;
use crate::core::enums::AppState;

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
    draw_training(f, app, chunks[1]);
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
                Constraint::Length(2),
                Constraint::Length(10),
                Constraint::Min(9),
            ]
            .as_ref(),
        )
        .split(area);

    let progress_widget = Gauge::default()
        .block(Block::default().title("Progress:"))
        .gauge_style(style.add_modifier(Modifier::ITALIC | Modifier::BOLD))
        .ratio(app.lesson_progress.progress());
    f.render_widget(progress_widget, chunks[0]);

    let statistics_widget = Table::new(
        vec![
            // Row can be created from simple strings.
            Row::new(vec!["WPM", "Errors"]),
            Row::new(vec![
                app.lesson_progress
                    .words_per_minute()
                    .unwrap_or(0)
                    .to_string(),
                app.lesson_progress.errors().to_string(),
            ]),
        ],
        // Paragraph::new(Text::styled(
        // app.lesson_progress
        //     .words_per_minute()
        //     .unwrap_or(0)
        //     .to_string(),
        // Style::default(),
    )
    .widths(&[Constraint::Length(5), Constraint::Length(5)])
    .block(Block::default().title("Table"))
    .style(style);
    f.render_widget(statistics_widget, chunks[1]);

    let lesson_text_widget = Paragraph::new(app.lesson_progress.get_diff());
    f.render_widget(lesson_text_widget, chunks[2]);
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
