use strum::{EnumMessage, IntoEnumIterator};
use strum_macros::{EnumIter, EnumMessage};

#[derive(EnumMessage, EnumIter, Debug)]
pub enum Command {
    #[strum(
        message = "start <word_count>",
        detailed_message = "Start a new solo game with <word_count> words."
    )]
    Start(usize),

    #[strum(message = "quit", detailed_message = "Quit TermRacer.")]
    Quit,

    #[strum(message = "help", detailed_message = "Print this help text.")]
    Help,
}

lazy_static! {
    pub static ref HELP_TEXT: String = {
        Command::iter()
            .map(|command| {
                format!(
                    "{}\n\t{}",
                    command.get_message().unwrap(),
                    command.get_detailed_message().unwrap()
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    };
}

pub fn parse(s: &str) -> Option<Command> {
    let args: Vec<&str> = s.split_whitespace().collect();

    match args.get(0) {
        Some(&"start") => match args.get(1) {
            Some(word_count_str) => word_count_str
                .parse::<usize>()
                .ok()
                .map(|word_count| Command::Start(word_count)),
            None => None,
        },
        Some(&"quit") => Some(Command::Quit),
        Some(&"help") => Some(Command::Help),
        _ => None,
    }
}
