use color_eyre::Result as CResult;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
};
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
