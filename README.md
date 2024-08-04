# Japanese-French Vocabulary

A command-line tool for learning Japanese words with French translations.

## Description

This Rust program helps users learn Japanese vocabulary by allowing them to add Japanese words with their French translations and take quizzes to test their knowledge. It uses a JSON file to store the vocabulary and provides two main functionalities:

1. Adding new words to the vocabulary list
2. Taking a quiz based on the stored words

## Features

- Add Japanese words with their French translations
- Store vocabulary in a JSON file
- Take a 10-question quiz with randomly selected words
- Immediate feedback on quiz answers
- Score tracking for quizzes

## Requirements

- Rust programming language (https://www.rust-lang.org/)
- Cargo package manager (comes with Rust)

## Installation

### From crates.io

```sh
cargo install nihongo
```

### From source

1. Clone this repository:

```sh
git clone https://github.com/yourusername/japanese-french-vocabulary.git cd japanese-french-vocabulary
```

2. Build the project:

```sh
cargo build --release
```

## Usage

### Adding a word

To add a new word to your vocabulary list:
cargo run -- add <japanese_word> <french_translation>

Example:

```sh
nihongo add
```

### Taking a quiz

To start a 10-question quiz:

```sh
nihongo quiz
```

Note: You need to have at least 10 words in your vocabulary list before you can take a quiz.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
