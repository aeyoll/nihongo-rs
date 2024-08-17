mod models;
mod srs;

use crate::srs::Srs;
use clap::{Parser, Subcommand};
use inquire::Text;

#[derive(Parser)]
#[clap(name = "Japanese-French Vocabulary")]
#[clap(author = "Jean-Philippe Bidegain")]
#[clap(version = "1.0")]
#[clap(about = "Learn Japanese words with French translations")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Add a new word")]
    Add,
    #[clap(about = "Start a quiz session")]
    Quiz {
        #[arg(short, long)]
        seed: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let srs = Srs::new();

    match &cli.command {
        Commands::Add => {
            let japanese = Text::new("Enter the Japanese word:").prompt().unwrap();
            let french = Text::new("Enter the French translation:").prompt().unwrap();

            srs.add_word(&japanese, &french);
        }
        Commands::Quiz { seed } => {
            srs.start_quiz(seed);
        }
    }
}
