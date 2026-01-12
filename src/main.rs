use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

/// XKCD-style passphrase generator with cryptographic security
#[derive(Parser)]
#[command(name = "rubs", version, about)]
struct Cli {
    /// Minimum bits of entropy for the passphrase
    #[arg(short = 'b', long = "bits", default_value = "64")]
    bits: u32,

    /// Path to a wordlist file (can be specified multiple times)
    ///
    /// If not specified, uses the bundled EFF large wordlist (7776 words).
    /// Multiple wordlists are merged together.
    #[arg(short = 'w', long = "wordlist", action = clap::ArgAction::Append)]
    wordlists: Option<Vec<PathBuf>>,

    /// Only output the passphrase (no statistics)
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,

    /// Launch interactive TUI mode
    #[arg(short = 't', long = "tui")]
    tui: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let wordlist = match &cli.wordlists {
        Some(paths) => {
            let path_refs: Vec<_> = paths.iter().map(|p| p.as_path()).collect();
            match rubs::load_wordlists(&path_refs) {
                Ok(words) => words,
                Err(e) => {
                    eprintln!("error: failed to load wordlist: {e}");
                    return ExitCode::FAILURE;
                }
            }
        }
        None => rubs::load_bundled(),
    };

    if wordlist.is_empty() {
        eprintln!("error: wordlist is empty (no valid words found)");
        return ExitCode::FAILURE;
    }

    if cli.tui {
        if let Err(e) = rubs::tui::run(wordlist, cli.bits) {
            eprintln!("error: TUI failed: {e}");
            return ExitCode::FAILURE;
        }
        return ExitCode::SUCCESS;
    }

    let info = rubs::generate(cli.bits, &wordlist);

    if cli.quiet {
        println!("{}", info.passphrase());
    } else {
        let source = if cli.wordlists.is_some() {
            "custom wordlist(s)"
        } else {
            "EFF large wordlist"
        };
        println!(
            "{} unique words from {} ({:.1} bits per word)",
            info.wordlist_size, source, info.bits_per_word
        );
        println!(
            "Requested {} bits; {} word(s) provide {:.1} bits:",
            cli.bits,
            info.words.len(),
            info.total_bits()
        );
        println!("{}", info.passphrase());
    }

    ExitCode::SUCCESS
}
