# rubs

A Rust port of [chubs](https://github.com/kwiberg/chubs), the XKCD-style passphrase generator.

Read the [original XKCD comic](http://xkcd.com/936/) to understand why passphrases beat passwords.

## What is this?

rubs generates secure passphrases by randomly selecting words from wordlists. Humans are notoriously bad at generating random sequences, so we delegate that job to cryptographically secure random number generators.

This project is a loving port of chubs from Python to Rust, built as a learning exercise. While faithful to the original's core functionality, rubs extends it with a TUI interface and additional cryptographic features.

## Planned Features

### Core (from chubs)
- Generate passphrases with configurable entropy (default: 64 bits)
- Support custom wordlists from any text file
- Bundled EFF Diceware wordlist (7776 words, ~12.9 bits per word)
- Fetch and cache wordlists from URLs (like Project Gutenberg texts)

### Extensions
- **TUI**: Interactive terminal interface built with ratatui
- **Entropy mixing**: Combine OS randomness with user-provided entropy (dice rolls, keyboard timing) via HKDF
- **Key derivation**: Derive cryptographic keys from passphrases using Argon2id
- **Encrypted storage**: Save wordlists or passphrase history encrypted with XChaCha20-Poly1305
- **Strength analysis**: Estimate passphrase security against common attack models

## Installation

Coming soon via Homebrew and GitHub releases.

## Usage

```bash
# Open TUI
rubs

# Generate passphrase (CLI mode)
rubs generate

# Generate with 128 bits of entropy
rubs generate -b 128

# Use a custom wordlist
rubs generate -w /path/to/wordlist.txt
```

## Acknowledgments

rubs exists because [chubs](https://github.com/kwiberg/chubs) exists. All credit for the original concept and implementation goes to Karl Wiberg.

## License

Dual-licensed under MIT or BSD-2-Clause, at your option. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-BSD-2-Clause](LICENSE-BSD-2-Clause).
