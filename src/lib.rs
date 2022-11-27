use std::{error::Error, fmt::Display, fs, io::Write};

use colored::*;

use clap::{Parser, Subcommand};
use dialoguer::MultiSelect;

/// Basic todo application
#[derive(Debug, Parser)]
#[command(name = "quests")]
#[command(about = "Basic todo application", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    List {
        /// Show all quests
        #[arg(short)]
        all: bool,

        /// Toggle interactively
        #[arg(short)]
        interactive: bool,
    },
    #[command(arg_required_else_help = true)]
    Add {
        /// Text of quest to add
        #[arg(short, long, required = true)]
        quest: String,
    },
}

const FINISHED_QUEST_PREFIX: &str = "- [x]";
const UNFINISHED_QUEST_PREFIX: &str = "- [ ]";
const QUESTS_FILE_PATH: &str = "quests.txt";

pub fn run(args: Cli) -> Result<(), Box<dyn Error>> {
    match args.command {
        Commands::Add { quest } => add_quest(quest)?,
        Commands::List { all, interactive } => {
            if all {
                list_quests(interactive)?
            } else {
                list_unfinished_quests()?
            }
        }
    }

    Ok(())
}

pub fn add_quest(quest: String) -> Result<(), Box<dyn Error>> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(QUESTS_FILE_PATH)
        .unwrap();

    if let Err(e) = file.write(format!("{}{}\n", UNFINISHED_QUEST_PREFIX, quest).as_bytes()) {
        return Err(e.into());
    }

    file.flush().expect("Flushing failed");

    println!("Added quest: {}", quest);

    Ok(())
}

pub fn list_quests(interactive: bool) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(QUESTS_FILE_PATH)?;

    if interactive {
        let mut items = contents
            .lines()
            .map(|x| {
                (
                    x.chars()
                        .into_iter()
                        .enumerate()
                        .skip_while(|&(i, _)| i < FINISHED_QUEST_PREFIX.len())
                        .map(|(_, x)| x)
                        .collect::<String>(),
                    x.starts_with(FINISHED_QUEST_PREFIX),
                )
            })
            .collect::<Vec<(String, bool)>>();

        items.sort_unstable_by_key(|x| x.1);

        let chosen_indexes: Vec<usize> = MultiSelect::new().items_checked(&items).interact()?;

        let items = items
            .into_iter()
            .enumerate()
            .map(|(i, (item, _))| (item, chosen_indexes.contains(&i)))
            .collect::<Vec<(String, bool)>>();

        save_quests(&items)?;
    } else {
        for line in contents.lines() {
            if line.starts_with(FINISHED_QUEST_PREFIX) {
                println!("{}", line.green());
            } else {
                println!("{}", line.yellow());
            }
        }
    }

    Ok(())
}

pub fn list_unfinished_quests() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(QUESTS_FILE_PATH)?;

    for line in contents
        .lines()
        .filter(|line| !line.starts_with(FINISHED_QUEST_PREFIX))
    {
        println!("{}", line.yellow());
    }

    Ok(())
}

pub fn save_quests<T: Display>(quests: &[(T, bool)]) -> Result<(), Box<dyn Error>> {
    let contents = quests
        .iter()
        .map(|(quest, finished)| {
            if *finished {
                format!("{}{}\n", FINISHED_QUEST_PREFIX, quest)
            } else {
                format!("{}{}\n", UNFINISHED_QUEST_PREFIX, quest)
            }
        })
        .collect::<String>();

    if let Err(e) = fs::write(QUESTS_FILE_PATH, contents.as_bytes()) {
        return Err(e.into());
    }

    Ok(())
}
