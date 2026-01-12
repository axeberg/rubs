//! Passphrase generation with entropy calculation.
//!
//! Uses cryptographically secure randomness from the OS via the `rand` crate.

use rand::prelude::IndexedRandom;
use std::collections::HashSet;

/// Information about a generated passphrase.
#[derive(Debug, Clone)]
pub struct PassphraseInfo {
    /// Total number of unique words in the source wordlist.
    pub wordlist_size: usize,
    /// Bits of entropy per word (log2 of wordlist size).
    pub bits_per_word: f64,
    /// The generated passphrase words.
    pub words: Vec<String>,
}

impl PassphraseInfo {
    /// Returns the total entropy of the passphrase in bits.
    pub fn total_bits(&self) -> f64 {
        self.words.len() as f64 * self.bits_per_word
    }

    /// Returns the passphrase as a space-separated string.
    pub fn passphrase(&self) -> String {
        self.words.join(" ")
    }
}

/// Generates a passphrase with at least the specified entropy bits.
///
/// Uses cryptographically secure randomness via `rand::rng()` which sources
/// entropy from the operating system (getrandom on Linux, SecRandomCopyBytes
/// on macOS, BCryptGenRandom on Windows).
///
/// # Panics
///
/// Panics if the wordlist is empty.
pub fn generate(bits: u32, wordlist: &HashSet<String>) -> PassphraseInfo {
    assert!(!wordlist.is_empty(), "wordlist cannot be empty");

    let bits_per_word = (wordlist.len() as f64).log2();
    let words_needed = (bits as f64 / bits_per_word).ceil() as usize;

    // Sort for deterministic ordering before sampling
    let mut sorted_words: Vec<_> = wordlist.iter().cloned().collect();
    sorted_words.sort();

    // Sample using OS-provided cryptographic randomness
    let mut rng = rand::rng();
    let chosen: Vec<String> = sorted_words
        .choose_multiple(&mut rng, words_needed)
        .cloned()
        .collect();

    PassphraseInfo {
        wordlist_size: wordlist.len(),
        bits_per_word,
        words: chosen,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_wordlist() -> HashSet<String> {
        (0..1000).map(|i| format!("word{i}")).collect()
    }

    #[test]
    fn generates_correct_word_count() {
        let wordlist = test_wordlist();
        let info = generate(64, &wordlist);

        // 1000 words = ~9.97 bits per word, so 64 bits needs 7 words
        assert!(info.total_bits() >= 64.0);
        assert_eq!(info.words.len(), 7);
    }

    #[test]
    fn entropy_calculation() {
        let wordlist: HashSet<_> = (0..7776).map(|i| format!("word{i}")).collect();
        let info = generate(64, &wordlist);

        // 7776 words = 12.925 bits per word
        let expected_bpw = (7776f64).log2();
        assert!((info.bits_per_word - expected_bpw).abs() < 0.001);
    }

    #[test]
    fn passphrase_string_format() {
        let wordlist = test_wordlist();
        let info = generate(32, &wordlist);

        let passphrase = info.passphrase();
        let word_count = passphrase.split_whitespace().count();
        assert_eq!(word_count, info.words.len());
    }

    #[test]
    #[should_panic(expected = "wordlist cannot be empty")]
    fn panics_on_empty_wordlist() {
        let wordlist = HashSet::new();
        generate(64, &wordlist);
    }
}
