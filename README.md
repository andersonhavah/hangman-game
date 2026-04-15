# Overview

This project is a **Hangman game** implemented in Rust, designed to deepen my understanding of core Rust language concepts and best practices. As a software engineer, I created this application to master fundamental programming paradigms while also building a functional, user-friendly command-line game.

## Purpose

The primary objective was to gain hands-on experience with Rust's unique approach to memory safety and ownership through practical implementation. By building a complete, playable game, I explored how Rust's compiler and type system enforce safe, concurrent code patterns without sacrificing performance.

## Features

This interactive Hangman game demonstrates:

- **Ownership & Borrowing**: Functions pass references to data structures rather than transferring ownership, showcasing Rust's core memory model
- **Data Structures**: Leverages `Vec` for tracking guessed letters and `HashMap` for efficient letter-position lookups
- **Pattern Matching**: Uses `match` expressions for elegant control flow
- **Mutable vs. Immutable Bindings**: Carefully controls state mutation for game progress tracking
- **Input Validation & Error Handling**: Robust user input processing with graceful error recovery
- **Random Selection**: Uses the `rand` crate to select words from a Rust-themed word list

## Gameplay

Players guess letters to reveal a hidden Rust-themed word before the hangman is drawn. With six wrong guesses allowed, players must strategically deduce the word. The game features visual ASCII art stages and clear feedback on each guess.

[Software Demo Video](https://youtu.be/3t1EyJaX53M)

# Development Environment

## Tools & Language

- **Language**: Rust (Edition 2021)
- **Package Manager**: Cargo (Rust's built-in build system)
- **Editor**: Visual Studio Code
- **Compiler**: rustc (Rust compiler)

## Dependencies

- **rand** (v0.8): Provides cryptographically secure random number generation for selecting words from the word list

The project structure is organized as a standard Rust binary crate, with all source code in `src/main.rs` and configuration in `Cargo.toml`.

# Useful Websites

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - Comprehensive guide to Rust fundamentals and idioms
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/) - Official API reference for core Rust libraries
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Practical, hands-on examples of Rust concepts
- [Rand Crate Documentation](https://docs.rs/rand/) - Guide to random number generation in Rust
- [Rustlings Course](https://github.com/rust-lang/rustlings) - Interactive exercises for learning Rust
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - Deep dive into unsafe Rust and advanced concepts

# Future Work

- **Difficulty Levels**: Implement easy, medium, and hard modes with different word lists and attempt limits
- **Word Categories**: Organize words by theme (animals, technology, geography) and let players choose categories
- **Statistics Tracking**: Keep a running score of wins/losses and player performance metrics
- **GUI Enhancement**: Build a graphical interface using a framework like `iced` or `fltk-rs`
- **Async Features**: Implement online multiplayer support using async Rust and networking libraries
- **Custom Word Lists**: Allow users to load custom word lists from files
- **Hint System**: Add limited hints during gameplay to increase engagement
- **Time Challenges**: Introduce a timed mode where players race against the clock
- **File I/O**: Save and load game progress for extended play sessions
