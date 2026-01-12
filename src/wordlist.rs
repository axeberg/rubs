//! Wordlist loading and filtering for passphrase generation.
//!
//! Supports loading words from external files as well as using the bundled
//! EFF large wordlist (7776 words, ~12.9 bits per word).

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

/// The bundled EFF large wordlist for diceware passphrases.
/// Format: tab-separated dice numbers and words, one per line.
const EFF_WORDLIST: &str = include_str!("../data/eff_large_wordlist.txt");

/// Returns true if the word contains only lowercase ASCII letters.
fn is_valid_word(word: &str) -> bool {
    !word.is_empty() && word.chars().all(|c| c.is_ascii_lowercase())
}

/// Loads words from the bundled EFF wordlist.
///
/// Extracts only the word portion (second column) from each line,
/// filtering to ensure only lowercase alphabetic words are included.
pub fn load_bundled() -> HashSet<String> {
    EFF_WORDLIST
        .lines()
        .filter_map(|line| {
            let word = line.split('\t').nth(1)?;
            let word = word.trim().to_lowercase();
            if is_valid_word(&word) {
                Some(word)
            } else {
                None
            }
        })
        .collect()
}

/// Loads words from an external file.
///
/// Splits on whitespace and filters to only include lowercase alphabetic words.
/// This matches the behavior of the original chubs implementation.
pub fn load_from_file(path: &Path) -> io::Result<HashSet<String>> {
    let content = fs::read_to_string(path)?;
    let words = content
        .split_whitespace()
        .map(|w| w.trim().to_lowercase())
        .filter(|w| is_valid_word(w))
        .collect();
    Ok(words)
}

/// Loads and merges words from multiple wordlist files.
///
/// If no paths are provided, returns the bundled EFF wordlist.
pub fn load_wordlists(paths: &[&Path]) -> io::Result<HashSet<String>> {
    if paths.is_empty() {
        return Ok(load_bundled());
    }

    let mut words = HashSet::new();
    for path in paths {
        let file_words = load_from_file(path)?;
        words.extend(file_words);
    }
    Ok(words)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_wordlist_loads() {
        let words = load_bundled();
        // EFF list has 7776 entries, but 4 contain hyphens (drop-down, felt-tip, t-shirt, yo-yo)
        // which are filtered out by our lowercase-only validation
        assert_eq!(
            words.len(),
            7772,
            "EFF large wordlist should have 7772 valid words"
        );
    }

    #[test]
    fn bundled_wordlist_contains_expected_words() {
        let words = load_bundled();
        assert!(words.contains("abacus"));
        assert!(words.contains("zoom"));
    }

    #[test]
    fn valid_word_checks() {
        assert!(is_valid_word("hello"));
        assert!(is_valid_word("abacus"));
        assert!(!is_valid_word("Hello")); // uppercase
        assert!(!is_valid_word("hello1")); // digit
        assert!(!is_valid_word("hello-world")); // hyphen
        assert!(!is_valid_word("")); // empty
    }
}
