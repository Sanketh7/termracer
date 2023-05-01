use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "TermRacer Client")]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  Solo {
    #[arg(short = 'w', long)]
    word_count: usize,
  },
}
