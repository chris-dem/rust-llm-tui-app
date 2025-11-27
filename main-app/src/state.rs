use color_eyre::Result as CResult;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use std::fmt::Display;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AppMode {
    #[default]
    NavigationMode,
    ChatMode,
}

impl AppMode {
    pub fn toggle(self) -> Self {
        match self {
            Self::ChatMode => Self::NavigationMode,
            Self::NavigationMode => Self::ChatMode,
        }
    }
}

impl Display for AppMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            Self::ChatMode => "Chat Mode",
            Self::NavigationMode => "Navigation Mode",
        };
        write!(f, "{txt}")
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessageSender {
    #[default]
    User,
    LLM,
}

impl Display for MessageSender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            Self::User => "User",
            Self::LLM => "LLM",
        };
        write!(f, "{txt}")
    }
}

#[derive(Debug, Default)]
pub struct App {
    // TODO: add
    // ollama_state: Ollama,
    chat: Vec<(MessageSender, String)>,
    is_polling: bool,
    app_mode: AppMode,
    exit: bool,
}

impl App {
    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> CResult<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn get_state(&self) -> AppMode {
        self.app_mode
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> CResult<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match (self.app_mode, key_event.code) {
            (AppMode::NavigationMode, KeyCode::Char('q')) => self.exit(),
            (_, KeyCode::Esc) => self.app_mode = self.app_mode.toggle(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    input_string: String,
    cursor_index: usize,
}

impl Input {
    const fn new() -> Self {
        Self {
            input_string: String::new(),
            cursor_index: 0,
        }
    }

    pub fn byte_index(&self) -> usize {
        self.input_string
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_index)
            .unwrap_or(self.input_string.len())
    }

    pub fn add_char(&mut self, input_char: char) {
        self.input_string.insert(self.cursor_index, input_char);
        self.move_right();
    }

    fn traverse(&mut self, left_right: bool) {
        let indx = if left_right {
            self.cursor_index.saturating_add(1)
        } else {
            self.cursor_index.saturating_sub(1)
        };
        self.cursor_index = self.clamp_cursor(indx)
    }

    pub fn move_left(&mut self) {
        self.traverse(false)
    }

    pub fn move_right(&mut self) {
        self.traverse(true)
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input_string.chars().count())
    }

    pub fn reset_cursor(&mut self, new_cursor_pos: usize) {
        self.cursor_index = 0;
    }

    fn delete_char(&mut self) {}
}
