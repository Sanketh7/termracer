#[derive(Debug)]
pub enum Command {
    Start(usize),
    Quit,
    Help,
}

pub const HELP_STRING: &str = "Commands:\n\
                            \tstart <num_words>\n\
                            \t\tstart a new session with <num_words> words\n\
                            \tquit\n\
                            \t\tquit TermRacer\n\
                            \thelp\n\
                            \t\tprint this help text";

pub fn parse_string(s: String) -> Option<Command> {
    let args: Vec<&str> = s.split_whitespace().collect();

    match args.get(0) {
        Some(&"start") => match args.get(1) {
            Some(num_words_str) => num_words_str
                .parse::<usize>()
                .ok()
                .map(|num_words| Command::Start(num_words)),
            None => None,
        },
        Some(&"quit") => Some(Command::Quit),
        Some(&"help") => Some(Command::Help),
        _ => None,
    }
}
