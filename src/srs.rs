use crate::models::Word;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use colored::Colorize;
use inquire::Text;
use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

static WORDS_FILE: &str = "words.json";

#[derive(Clone, Copy)]
pub struct Srs {}

impl Srs {
    pub fn new() -> Srs {
        Srs {}
    }

    /// Get the words from the file
    fn get_words_from_file(self) -> Vec<Word> {
        if Path::new(WORDS_FILE).exists() {
            // File exists, read its contents
            let file = File::open(WORDS_FILE).expect("Unable to open file");
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
        } else {
            // File doesn't exist, start with an empty vector
            Vec::new()
        }
    }

    /// Save the words to the file
    fn save_words_to_file(self, words: &[Word]) {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(WORDS_FILE)
            .expect("Unable to open file for writing");

        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &words).expect("Unable to write to file");
    }

    /// Select the quiz words
    /// Each word has a weight based on its success rate and the number of tries it has received
    /// The weight is calculated as follows:
    /// weight = 1 - (success_rate / tries)
    /// The total weight is calculated as the sum of all weights
    /// The random value is then generated between 0 and the total weight
    /// Each word is then selected until the random value is less than its weight
    fn select_quiz_words(self, words: &[Word], count: usize) -> Vec<&Word> {
        let mut rng = rand::thread_rng();
        let mut selected_words = Vec::new();
        let mut selected_indexes: Vec<usize> = Vec::new();

        let mut i = count;

        while i > 0 {
            let total_weight: f64 = words
                .iter()
                .map(|w| 1.0 - (w.success as f64 / w.tries.max(1) as f64))
                .sum();

            let mut random_value = rng.gen::<f64>() * total_weight;

            for (index, word) in words.iter().enumerate() {
                let weight = 1.0 - (word.success as f64 / word.tries.max(1) as f64);

                if (random_value <= weight || word.tries == 0) && !selected_words.contains(&word) {
                    selected_words.push(word);
                    selected_indexes.push(index);
                    i -= 1;
                    break;
                }

                random_value -= weight;
            }
        }

        // Print the seed
        let seed_as_string = selected_indexes
            .iter()
            .map(|&i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let seed_as_base64 = URL_SAFE.encode(seed_as_string);
        println!("Seed: {}", seed_as_base64);

        selected_words
    }

    /// Add a new word to the vocabulary list
    pub fn add_word(self, japanese: &str, french: &str) {
        let mut words: Vec<Word> = self.get_words_from_file();

        let word = Word {
            japanese: japanese.to_string(),
            french: french.to_string(),
            success: 0,
            tries: 0,
        };

        // Add the new word if it doesn't already exist
        if !words.iter().any(|w| w.japanese == word.japanese) {
            words.push(word);

            // Write the updated words list back to the file
            self.save_words_to_file(&words);

            println!("{} Word added successfully! 🎉", "Success:".green().bold());
        } else {
            println!("{} Word already exists! 😕", "Warning:".yellow().bold());
        }
    }

    /// Start a quiz session
    /// The quiz consists of 10 questions, each with a random selection of words from the vocabulary list
    /// The user is asked to answer the question and the correct answer is displayed
    /// If the user's answer is correct, their score is incremented
    /// The quiz is then repeated until the user has scored at least 8 points
    pub fn start_quiz(self, seed: &Option<String>) {
        let mut words: Vec<Word> = self.get_words_from_file();

        if words.len() < 10 {
            println!("Not enough words for a quiz. Please add at least 10 words.");
            return;
        }

        let base_words = words.clone();

        // Get the quiz words from the seed if provided
        let quiz_words = if let Some(seed) = seed {
            let seed_as_bytes = URL_SAFE.decode(seed).unwrap();
            let seed_as_string = String::from_utf8(seed_as_bytes).unwrap();
            let seed_as_numbers: Vec<usize> = seed_as_string
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            seed_as_numbers
                .iter()
                .map(|&i| &base_words[i])
                .collect::<Vec<&Word>>()
        } else {
            self.select_quiz_words(&base_words, 10)
        };

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
        self.save_words_to_file(&words);

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
}
