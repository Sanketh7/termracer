#[derive(Debug)]
pub enum Command {
    Start,
    Quit,
}

pub fn parse_string(s: String) -> Option<Command> {
    let args: Vec<&str> = s.split_whitespace().collect();

    match args.get(0) {
        Some(&"start") => Some(Command::Start),
        Some(&"quit") => Some(Command::Quit),
        _ => None,
    }
}
