//! rubs - XKCD-style passphrase generator
//!
//! A Rust implementation inspired by [chubs](https://github.com/kwiberg/chubs),
//! generating secure passphrases using cryptographically secure randomness.

pub mod generate;
pub mod tui;
pub mod wordlist;

pub use generate::{PassphraseInfo, generate};
pub use wordlist::{load_bundled, load_from_file, load_wordlists};
