use crate::models::Card;
use chrono::{Duration, Utc};
use colored::Colorize;
use inquire::Text;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

static CARDS_FILE: &str = "cards.json";

#[derive(Clone, Deserialize, Serialize)]
pub struct LeitnerSystem {
    cards: HashMap<String, Card>,
    boxes: usize,
}

impl LeitnerSystem {
    pub fn new() -> LeitnerSystem {
        let new = LeitnerSystem {
            cards: HashMap::new(),
            boxes: 5,
        };

        if Path::new(CARDS_FILE).exists() {
            // File exists, read its contents
            let file = File::open(CARDS_FILE).expect("Unable to open file");
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or(new)
        } else {
            // File doesn't exist, start with an empty vector
            new
        }
    }

    /// Save the cards to the file
    fn save_cards_to_file(self) {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(CARDS_FILE)
            .expect("Unable to open file for writing");

        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self).expect("Unable to write to file");
    }

    /// Get the due cards by next review date
    fn get_due_cards(&self, count: &usize) -> Vec<Card> {
        let due_cards: Vec<Card> = self
            .cards
            .values()
            .filter(|card| card.next_review <= Utc::now())
            .take(*count)
            .cloned()
            .collect();

        due_cards
    }

    /// Review a card and move it to the next box if correct,
    /// or to the first box if incorrect
    /// The next review date is updated depending on the box number
    fn review_card(&mut self, question: &str, correct: bool) {
        if let Some(card) = self.cards.get_mut(question) {
            if correct {
                if card.box_number < self.boxes - 1 {
                    card.box_number += 1;
                }
            } else {
                card.box_number = 0;
            }

            // Set next review time based on new box
            card.next_review = Utc::now() + Duration::days(2_i64.pow(card.box_number as u32));
        }
    }

    /// Add a new card to the vocabulary list and save it to the file
    pub fn add_card(mut self, japanese: &str, french: &str) -> Result<String, anyhow::Error> {
        if self.cards.contains_key(japanese) {
            return Err(anyhow::anyhow!(
                "{} Card already exists! 😕",
                "Warning:".yellow().bold()
            ));
        }

        let card = Card {
            japanese: japanese.to_string(),
            french: french.to_string(),
            box_number: 0,
            next_review: Utc::now(),
        };
        self.cards.insert(japanese.to_string(), card);
        self.save_cards_to_file();

        Ok(format!(
            "{} Card added successfully! 🎉",
            "Success:".green().bold()
        ))
    }

    /// Start a quiz session and save the results to the file
    pub fn start_quiz(mut self, count: &usize) -> Result<String, anyhow::Error> {
        let quiz_words = self.get_due_cards(count);

        if quiz_words.is_empty() {
            return Err(anyhow::anyhow!("No cards due for quiz! 😕"));
        }

        if quiz_words.len() < *count {
            return Err(anyhow::anyhow!("Not enough cards due for quiz! 😕"));
        }

        let mut score = 0;

        for (i, word) in quiz_words.iter().enumerate() {
            println!("\nQuestion {} of {}:", i + 1, count);

            let answer = Text::new(&format!(
                "What's the French translation of '{}'?",
                word.japanese.yellow()
            ))
            .prompt()?;

            let correct = answer.to_lowercase() == word.french.to_lowercase();

            if correct {
                println!("{} 🎉", "Correct!".green().bold());
                score += 1;
            } else {
                println!(
                    "{} The correct answer is: {}",
                    "Incorrect.".red().bold(),
                    word.french.green()
                );
            }

            self.review_card(&word.japanese, correct);
        }

        // Save updated cards stats
        self.save_cards_to_file();

        // Calculate the quiz ratio
        let ratio = score as f64 / *count as f64;

        let message = format!(
            "\n{} Your score: {} out of {} {}",
            "Quiz completed!".blue().bold(),
            score.to_string().yellow().bold(),
            count,
            if ratio >= 0.8 {
                "🏆"
            } else if ratio >= 0.5 {
                "👍"
            } else {
                "🌱"
            }
        );

        Ok(message)
    }

    /// List all cards in the vocabulary list
    pub fn list_cards(&self) -> Result<String, anyhow::Error> {
        let mut message = String::new();
        let mut sorted_cards: Vec<_> = self.cards.iter().collect();
        sorted_cards.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (japanese, card) in sorted_cards {
            message.push_str(&format!("{}: {}\n", japanese.yellow(), card.french.green()));
        }

        Ok(message)
    }
}
