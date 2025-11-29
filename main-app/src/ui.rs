use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};
use std::str::FromStr;

use crate::state::{App, AppMode, Input};

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

impl App {
    fn bottom_rendering(&self, area: Rect, buf: &mut Buffer) {
        let lay_bot = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        self.get_state().render(lay_bot[0], buf);
    }

    fn up_rendering(&self, area: Rect, buf: &mut Buffer) {
        let lay_bot = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        self.get_state().render(lay_bot[0], buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Personal Agentic LLM ".bold());
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
            .split(area);
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .render(layout[0], buf);
        self.bottom_rendering(layout[1], buf);
    }
}

impl Widget for &Input {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Input Field ".bold());
        let block = Block::default().borders(Borders::ALL).title(title);
        Paragraph::new(self.input_string.as_str())
            .centered()
            .block(block)
            .style(Style::default().fg(Color::Yellow))
            .render(area, buf);
    }
}
