use crossterm::{execute, style};
use solo_game::{SoloGame, SoloGameResults};
use std::io;
use std::time::Duration;

mod layout;
mod rect;
mod solo_game;
mod throttler;
mod views;
mod window;

fn main() {
    let mut buf = io::stdout().lock();

    let mut game = SoloGame::new();
    let game_results = game.run(&mut buf, Duration::from_millis(1000 / 30));

    let end_text = match game_results {
        SoloGameResults::Completed { wpm } => format!("WPM: {}\n", wpm as u32),
        SoloGameResults::Aborted => "Aborted!\n".to_string(),
    };
    execute!(buf, style::Print(end_text)).expect("ERROR: Failed to print end text.");
}
