use std::io;
use std::time::Duration;

use clap::Parser;
use crossterm::{execute, style};
use util::cli::{Cli, Commands};

use crate::game::solo_game::SoloGame;
use crate::models::game_result::GameResult;

mod framework;
mod game;
mod models;
mod util;
mod views;

fn main() {
  let cli = Cli::parse();

  match cli.command {
    Commands::Solo { word_count } => {
      let mut buf = io::stdout().lock();

      let mut game = SoloGame::new(word_count);
      let game_results = game.run(&mut buf, Duration::from_millis(1000 / 30));

      let end_text = match game_results {
        GameResult::Completed { wpm } => format!("WPM: {}\n", wpm as u32),
        GameResult::Aborted => "Aborted!\n".to_string(),
      };
      execute!(buf, style::Print(end_text)).expect("ERROR: Failed to print end text.");
    }
  }
}
