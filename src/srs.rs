use crate::models::Word;
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

    fn select_quiz_words(self, words: &[Word], count: usize) -> Vec<&Word> {
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

    pub fn add_word(self, japanese: &str, french: &str) {
        let mut words: Vec<Word> = self.get_words_from_file();

        let word = Word {
            japanese: japanese.to_string(),
            french: french.to_string(),
            success: 0,
            tries: 0,
        };

        // Add the new word
        words.push(word);

        // Write the updated words list back to the file
        self.save_words_to_file(&words);

        println!("{} Word added successfully! 🎉", "Success:".green().bold());
    }

    pub fn deduplicate(self) {
        let mut words: Vec<Word> = self.get_words_from_file();

        let mut unique_words = Vec::new();

        for word in words.iter() {
            if !unique_words.contains(&word.japanese) {
                unique_words.push(word.japanese.clone());
            }
        }

        words.retain(|w| unique_words.contains(&w.japanese));

        self.save_words_to_file(&words);
    }

    pub fn start_quiz(self) {
        let mut words: Vec<Word> = self.get_words_from_file();

        if words.len() < 10 {
            println!("Not enough words for a quiz. Please add at least 10 words.");
            return;
        }

        let base_words = words.clone();
        let quiz_words = self.select_quiz_words(&base_words, 10);
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
