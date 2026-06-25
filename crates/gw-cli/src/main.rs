//! 'gw-cli' is the thin "single-shot command" interface described in
//! phase 1 of the roadmap. It should contain almost no logic itself --
//! just argument parsing and printing. The real work happens in
//! 'gw-core', so that the (future) TUI crate can resuse it directly.

use clap::{Parser, Subcommand};
use gw_core::{lookup, parse_reference};

/// A command line tool for looking up and searching Bible passages.
#[derive(Debug, Parser)]
#[command(name = "gw", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Look up a single reference, e.g. 'gw get "John 3:16"'
    Get {
        /// The reference to look up, e.g. "John 3:16"
        reference: String,
    },

    /// Search across the text for a word or phrase. (Not implemented yet --
    /// this is a placeholder for phase 2 of the roadmap.)
    Search {
        /// The text to search for
        query: String,
    },

    /// Print a random verse. Not implemented yet -- placeholder.)
    Random,
}
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Get { reference } => {
            let parsed = parse_reference(&reference)?;
            let text = lookup(&parsed)?;
            println!("{text}");
        }
        Command::Search { query } => {
            println!("searching for '{query}' isn't implemented yet (phase 2)");
        }
        Command::Random => {
            println!("random verse isn't implemented yet (phase 2)");
        }
    }

    Ok(())
}
