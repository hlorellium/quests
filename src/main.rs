use std::process;

use clap::Parser;
use quests::Cli;

fn main() {
    let args = Cli::parse();

    if let Err(e) = quests::run(args) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
