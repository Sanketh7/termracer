use crossterm::{execute, terminal};
use solo_game::SoloGame;
use std::io;
use std::time::Duration;

mod layout;
mod rect;
mod solo_game;
mod views;
mod window;

fn main() {
    let mut buf = io::stdout();
    execute!(buf, terminal::EnterAlternateScreen)
        .expect("ERROR: Failed to enter alternate screen.");
    terminal::enable_raw_mode().expect("ERROR: Failed to enable raw mode.");

    let mut game = SoloGame::new();
    game.game_loop(&mut buf, Duration::from_millis(35));

    terminal::disable_raw_mode().expect("ERROR: Failed to disable raw mode.");
    execute!(buf, terminal::LeaveAlternateScreen)
        .expect("ERROR: Failed to leave alternate screen.");
}
