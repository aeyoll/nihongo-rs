use clap::{Parser, Subcommand};
use inquire::{Text, Select};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use rand::seq::SliceRandom;

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

#[derive(Serialize, Deserialize, Debug)]
struct Word {
    japanese: String,
    french: String,
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

    println!("Word added successfully!");
}

fn start_quiz() {
    let file = OpenOptions::new()
        .read(true)
        .open("words.json")
        .expect("Unable to open file");

    let reader = BufReader::new(file);
    let words: Vec<Word> = serde_json::from_reader(reader).expect("Unable to deserialize");

    if words.len() < 10 {
        println!("Not enough words for a quiz. Please add at least 10 words.");
        return;
    }

    let mut rng = rand::thread_rng();
    let quiz_words: Vec<&Word> = words.choose_multiple(&mut rng, 10).collect();
    let mut score = 0;

    for (i, word) in quiz_words.iter().enumerate() {
        println!("\nQuestion {} of 10:", i + 1);

        let answer = Text::new(&format!("What's the French translation of '{}'?", word.japanese))
            .prompt()
            .unwrap();

        if answer.to_lowercase() == word.french.to_lowercase() {
            println!("Correct!");
            score += 1;
        } else {
            println!("Incorrect. The correct answer is: {}", word.french);
        }
    }

    println!("\nQuiz completed! Your score: {} out of 10", score);
}
