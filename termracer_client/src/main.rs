#[macro_use]
extern crate lazy_static;

use crossterm::{execute, style, ExecutableCommand};
use solo_game::{SoloGame, SoloGameResults};
use std::io::{self, Write};
use std::time::Duration;

use crate::command::Command;

mod command;
mod layout;
mod rect;
mod solo_game;
mod throttler;
mod views;
mod window;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("ERROR: Failed to read line.");
        line = line.trim_end().to_string();

        match command::parse(&line) {
            Some(Command::Start(word_count)) => {
                let mut buf = io::stdout().lock();

                let mut game = SoloGame::new(word_count);
                let game_results = game.run(&mut buf, Duration::from_millis(1000 / 30));

                let end_text = match game_results {
                    SoloGameResults::Completed { wpm } => format!("WPM: {}\n", wpm as u32),
                    SoloGameResults::Aborted => "Aborted!\n".to_string(),
                };
                execute!(buf, style::Print(end_text)).expect("ERROR: Failed to print end text.");
            }
            Some(Command::Help) => println!("{}", *command::HELP_TEXT),
            Some(Command::Quit) => break,
            None => println!("No such command.\nUse `help` to see a list of commands."),
        }
    }
}
