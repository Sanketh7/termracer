#[derive(Debug)]
pub enum Command {
    StartSession,
    Quit,
}

pub fn parse_string(s: String) -> Option<Command> {
    let args: Vec<&str> = s.split_whitespace().collect();

    match args.get(0) {
        Some(&"start-session") => Some(Command::StartSession),
        Some(&"quit") => Some(Command::Quit),
        _ => None,
    }
}
