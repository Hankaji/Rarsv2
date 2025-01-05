use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Commands {
    Quit,
}
