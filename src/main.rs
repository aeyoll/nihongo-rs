mod models;
mod srs;

use crate::srs::LeitnerSystem;
use clap::{Parser, Subcommand};
use inquire::Text;
use log::error;
use std::process;

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
        #[arg(short, long, default_value = "10")]
        count: usize,
    },
    List,
}

fn app() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    let srs = LeitnerSystem::new();

    match &cli.command {
        Commands::Add => {
            let japanese = Text::new("Enter the Japanese word:").prompt().unwrap();
            let french = Text::new("Enter the French translation:").prompt().unwrap();

            match srs.add_card(&japanese, &french) {
                Ok(message) => println!("{}", message),
                Err(err) => println!("{}", err),
            }
        }
        Commands::Quiz { count } => match srs.start_quiz(count) {
            Ok(message) => println!("{}", message),
            Err(err) => println!("{}", err),
        },
        Commands::List => match srs.list_cards() {
            Ok(message) => println!("{}", message),
            Err(err) => println!("{}", err),
        },
    }

    Ok(())
}

fn main() {
    process::exit(match app() {
        Ok(_) => 0,
        Err(err) => {
            error!("{}", err.to_string());
            1
        }
    });
}
