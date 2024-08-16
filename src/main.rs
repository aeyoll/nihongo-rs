use clap::{Parser, Subcommand};
use colored::*;
use inquire::Text;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

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
    Quiz,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Word {
    japanese: String,
    french: String,
    success: u32,
    tries: u32,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add => {
            let japanese = Text::new("Enter the Japanese word:").prompt().unwrap();
            let french = Text::new("Enter the French translation:").prompt().unwrap();

            add_word(&japanese, &french);
        }
        Commands::Quiz => {
            start_quiz();
        }
    }
}

fn add_word(japanese: &str, french: &str) {
    let word = Word {
        japanese: japanese.to_string(),
        french: french.to_string(),
        success: 0,
        tries: 0,
    };

    let file_path = "words.json";
    let mut words: Vec<Word> = if Path::new(file_path).exists() {
        // File exists, read its contents
        let file = File::open(file_path).expect("Unable to open file");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
    } else {
        // File doesn't exist, start with an empty vector
        Vec::new()
    };

    // Add the new word
    words.push(word);

    // Write the updated words list back to the file
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .expect("Unable to open file for writing");

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &words).expect("Unable to write to file");

    println!("{} Word added successfully! 🎉", "Success:".green().bold());
}

fn select_quiz_words(words: &[Word], count: usize) -> Vec<&Word> {
    let mut rng = rand::thread_rng();
    let mut selected_words = Vec::new();

    let mut i = count;

    while i > 0 {
        let total_weight: f64 = words
            .iter()
            .map(|w| 1.0 - (w.success as f64 / w.tries.max(1) as f64))
            .sum();

        let mut random_value = rng.gen::<f64>() * total_weight;

        for word in words {
            let weight = 1.0 - (word.success as f64 / word.tries.max(1) as f64);

            if (random_value <= weight || word.tries == 0) && !selected_words.contains(&word) {
                selected_words.push(word);
                i -= 1;
                break;
            }

            random_value -= weight;
        }
    }

    selected_words
}

fn start_quiz() {
    let file_path = "words.json";
    let file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .expect("Unable to open file");

    let reader = BufReader::new(file);
    let mut words: Vec<Word> = serde_json::from_reader(reader).expect("Unable to deserialize");

    if words.len() < 10 {
        println!("Not enough words for a quiz. Please add at least 10 words.");
        return;
    }

    let base_words = words.clone();
    let quiz_words = select_quiz_words(&base_words, 10);
    let mut score = 0;

    for (i, word) in quiz_words.iter().enumerate() {
        println!("\nQuestion {} of 10:", i + 1);

        let answer = Text::new(&format!(
            "What's the French translation of '{}'?",
            word.japanese.yellow()
        ))
        .prompt()
        .unwrap();

        // Find the word in the original list to update its stats
        if let Some(original_word) = words.iter_mut().find(|w| w.japanese == word.japanese) {
            original_word.tries += 1;

            if answer.to_lowercase() == word.french.to_lowercase() {
                println!("{} 🎉", "Correct!".green().bold());
                score += 1;
                original_word.success += 1;
            } else {
                println!(
                    "{} The correct answer is: {}",
                    "Incorrect.".red().bold(),
                    word.french.green()
                );
            }
        }
    }

    // Save updated word stats
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("Unable to open file for writing");

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &words).expect("Unable to write to file");

    println!(
        "\n{} Your score: {} out of 10 {}",
        "Quiz completed!".blue().bold(),
        score.to_string().yellow().bold(),
        if score >= 8 {
            "🏆"
        } else if score >= 5 {
            "👍"
        } else {
            "🌱"
        }
    );
}
