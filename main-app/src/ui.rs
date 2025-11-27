use color_eyre::{Result as CResult, owo_colors::OwoColorize};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};
use std::{fmt::Display, str::FromStr};

use crate::state::{App, AppMode};

impl Widget for AppMode {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from("App Mode".bold());
        let block = Block::bordered()
            .title(title.centered())
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let c = match self {
            Self::ChatMode => Color::from_str("lightblue"),
            Self::NavigationMode => Color::from_str("green"),
        }
        .expect("String Conversion to work");

        let counter_text = Text::from(vec![Line::from(vec![
            "Mode: ".to_string().bold(),
            self.to_string().italic(),
        ])])
        .centered()
        .fg(c);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Personal Agentic LLM ".bold());
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
            .split(area);
        let _block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .render(layout[0], buf);
        let lay_bot = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[1]);
        self.get_state().render(lay_bot[0], buf);
    }
}
