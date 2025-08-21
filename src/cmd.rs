use std::sync::{Arc, Mutex};
use colored::ColoredString;

pub struct CommandLine {
    style: Arc<Mutex<ColoredString>>,
    text: Arc<Mutex<ColoredString>>,
}

impl CommandLine {
    pub fn new(style: Arc<Mutex<ColoredString>>) -> CommandLine {
        CommandLine {
            style,
            text: Arc::new(Mutex::new("".into()))
        }
    }

    pub fn change_style(&mut self, style: ColoredString) -> &mut CommandLine {
        self.style = Arc::new(Mutex::new(style));

        self
    }

    pub fn change_text(&mut self, text: impl std::fmt::Display) -> &mut CommandLine {
        self.text = Arc::new(Mutex::new(text.to_string().into()));

        self
    }

    pub fn row(&self) -> (impl std::fmt::Display, impl std::fmt::Display) {
        (self.style.lock().unwrap().clone(), self.text.lock().unwrap().clone())
    }
}

