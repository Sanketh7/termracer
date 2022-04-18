use crossterm::{
    event,
    event::{Event, KeyCode},
};
use std::error::Error;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

pub fn spawn_thread(transmitter: Sender<KeyCode>) -> Result<JoinHandle<()>, Box<dyn Error>> {
    let handle = thread::spawn(move || loop {
        if let Event::Key(key_event) = event::read().unwrap() {
            transmitter.send(key_event.code).unwrap();
            if let KeyCode::Esc = key_event.code {
                break;
            }
        }
    });
    Ok(handle)
}
