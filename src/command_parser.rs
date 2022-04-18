#[derive(Debug)]
pub enum Command {
    Start,
    Quit,
    Help,
}

pub const HELP_STRING: &str = "Commands:\n\
                            \tstart\n\
                            \t\tstart a new session\n\
                            \tquit\n\
                            \t\tquit TermRacer\n\
                            \thelp\n\
                            \t\tprint this help text";

pub fn parse_string(s: String) -> Option<Command> {
    let args: Vec<&str> = s.split_whitespace().collect();

    match args.get(0) {
        Some(&"start") => Some(Command::Start),
        Some(&"quit") => Some(Command::Quit),
        Some(&"help") => Some(Command::Help),
        _ => None,
    }
}
