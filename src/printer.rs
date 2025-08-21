use std::io::Write;
use std::string::ToString;
use std::sync::{Arc};
use std::thread::sleep;
use std::time::Duration;
use crossterm::{cursor, execute};
use crossterm::terminal::{Clear, ClearType};
use crate::cmd::CommandLine;

pub fn print_one_by_one(cmd: &CommandLine, total_time: Duration, newline: bool) -> Arc<dyn Fn() + Send + Sync> {


    let (style, text) = cmd.row();
    let (style, text) = (style.to_string(), text.to_string());
    let break_secs = total_time.as_secs_f32() / text.len() as f32;

    Arc::new(move || {
        print!("{} ", style);
        for (index, ch) in text.chars().enumerate() {
            print!("{}", ch);
            std::io::stdout().flush().unwrap();

            if index != text.len() - 1 {
                sleep(Duration::from_secs_f32(break_secs));
            }
        }

        if newline {
            println!()
        }
    })
}

pub fn print_directly(cmd: &CommandLine, newline: bool) -> Arc<dyn Fn() + Send + Sync> {
    let (style, text) = cmd.row();
    let (style, text) = (style.to_string(), text.to_string());

    Arc::new(move || {
        print!("{} {}", style, text);
        std::io::stdout().flush().unwrap();

        if newline {
            println!()
        }
    })
}

pub fn newline() -> Arc<dyn Fn() + Send + Sync> {
    Arc::new(move || {
        println!()
    })
}

pub fn clear_all() -> Arc<dyn Fn() + Send + Sync> {
    Arc::new(move || {
        execute!(
            std::io::stdout(),
            Clear(ClearType::All),
        ).unwrap();

        execute!(
            std::io::stdout(),
            cursor::MoveTo(0, 0),
        ).unwrap();
    })
}

pub fn clear_line() -> Arc<dyn Fn() + Send + Sync> {
    Arc::new(move || {
        execute!(
            std::io::stdout(),
            Clear(ClearType::CurrentLine),
        ).unwrap();

        execute!(
            std::io::stdout(),
            cursor::MoveToColumn(0),
        ).unwrap();
    })
}