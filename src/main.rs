// ============================================================
//  Hangman in Rust
//  Demonstrates: variables (mut/immutable), expressions,
//  conditionals, loops, functions (ownership & references),
//  and data structures (Vec, HashMap)
// ============================================================

use std::collections::HashMap;
use std::io::{self, Write};
use rand::Rng;

// ── Constants (immutable bindings) ──────────────────────────
const MAX_WRONG: u8 = 6;

// Hangman ASCII art stages (immutable array of &str)
const STAGES: [&str; 7] = [
    r"
  +---+
  |   |
      |
      |
      |
      |
=========",
    r"
  +---+
  |   |
  O   |
      |
      |
      |
=========",
    r"
  +---+
  |   |
  O   |
  |   |
      |
      |
=========",
    r"
  +---+
  |   |
  O   |
 /|   |
      |
      |
=========",
    r"
  +---+
  |   |
  O   |
 /|\  |
      |
      |
=========",
    r"
  +---+
  |   |
  O   |
 /|\  |
 /    |
      |
=========",
    r"
  +---+
  |   |
  O   |
 /|\  |
 / \  |
      |
=========",
];

// ── Word list stored in a Vec<&str> ─────────────────────────
fn word_list() -> Vec<&'static str> {
    vec![
        "ownership", "borrowing", "lifetime", "closure",
        "iterator", "pattern", "matching", "concurrency",
        "fearless", "compiler", "inference", "polymorphism",
        "abstraction", "immutable", "mutable", "fibonacci",
        "algorithm", "recursion", "hashmap", "vector",
        "structure", "enumerate", "rustacean", "ferris",
    ]
}

// ── Select a random word (returns owned String) ──────────────
fn pick_word(words: &[&str]) -> String {
    let idx = rand::thread_rng().gen_range(0..words.len());
    words[idx].to_string()
}

// ── Build a HashMap: letter → list of positions ──────────────
//   e.g. "hello" → { 'h':[0], 'e':[1], 'l':[2,3], 'o':[4] }
fn build_letter_map(word: &str) -> HashMap<char, Vec<usize>> {
    let mut map: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, ch) in word.chars().enumerate() {
        map.entry(ch).or_insert_with(Vec::new).push(i);
    }
    map
}

// ── Build the display string showing guessed letters ─────────
//   Passes references to avoid ownership transfer
fn build_display(word: &str, guessed: &[char]) -> String {
    word.chars()
        .map(|ch| {
            if guessed.contains(&ch) { ch.to_string() } else { '_'.to_string() }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

// ── Check whether the player has won ────────────────────────
fn has_won(word: &str, guessed: &[char]) -> bool {
    word.chars().all(|ch| guessed.contains(&ch))
}

// ── Print the current game state ────────────────────────────
fn print_state(wrong: u8, display: &str, guessed: &[char]) {
    println!("{}", STAGES[wrong as usize]);
    println!("\n  Word:  {}", display);
    let guessed_str: String = guessed.iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!("  Guessed: [{}]", guessed_str);
    println!("  Wrong guesses left: {}", MAX_WRONG - wrong);
    println!("{}", "-".repeat(40));
}

// ── Read and validate a single character from stdin ──────────
fn read_guess() -> char {
    loop {
        print!("  Enter a letter: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let trimmed = input.trim().to_lowercase();
        if trimmed.len() == 1 {
            let ch = trimmed.chars().next().unwrap();
            if ch.is_alphabetic() {
                return ch;
            }
        }
        println!("  ⚠  Please enter a single letter (a–z).");
    }
}

// ── Core game logic — returns true if player won ─────────────
fn play_game(word: &str) -> bool {
    // Build the letter→positions map (HashMap)
    let letter_map = build_letter_map(word);

    // Mutable state
    let mut guessed: Vec<char> = Vec::new();   // Vec tracking all guesses
    let mut wrong_count: u8 = 0;               // mutable wrong-guess counter

    // Main game loop
    loop {
        let display = build_display(word, &guessed);
        print_state(wrong_count, &display, &guessed);

        // Win condition
        if has_won(word, &guessed) {
            println!("\n  🎉  You won! The word was \"{}\".\n", word);
            return true;
        }

        // Loss condition
        if wrong_count >= MAX_WRONG {
            println!("\n  💀  Game over! The word was \"{}\".\n", word);
            return false;
        }

        let guess = read_guess();

        // Duplicate guess check (conditional + Vec reference)
        if guessed.contains(&guess) {
            println!("  ↩  You already guessed '{}'.", guess);
            continue;
        }

        guessed.push(guess);   // ownership: push moves char into Vec

        // Check guess against HashMap
        if letter_map.contains_key(&guess) {
            let positions = &letter_map[&guess];
            println!("  ✓  '{}' is in the word at position(s): {:?}", guess, positions);
        } else {
            wrong_count += 1;   // mutable variable update
            println!("  ✗  '{}' is not in the word.", guess);
        }

        println!();
    }
}

// ── Ask the player if they want to play again ────────────────
fn ask_replay() -> bool {
    loop {
        print!("  Play again? (y/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no"  => return false,
            _           => println!("  Please enter 'y' or 'n'."),
        }
    }
}

// ── Entry point ──────────────────────────────────────────────
fn main() {
    // Immutable word list
    let words = word_list();

    println!("\n╔══════════════════════════════════════╗");
    println!("║                HANGMAN               ║");
    println!("╚══════════════════════════════════════╝");
    println!("  Guess the hidden Rust-themed word.");
    println!("  You have {} wrong guesses before the\n  hangman is complete.\n", MAX_WRONG);

    // Replay loop
    loop {
        let word = pick_word(&words);   // owned String returned
        play_game(&word);               // pass reference to game fn

        if !ask_replay() {
            println!("\n  Thanks for playing! Goodbye.\n");
            break;
        }
    }
}