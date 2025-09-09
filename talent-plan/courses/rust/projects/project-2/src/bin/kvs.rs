use std::path::Path;

use clap::Parser;
use clap::{Args, Subcommand, ValueEnum};
use kvs::{KvStore, Result};

#[derive(Parser, Debug)]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    // Set value to a key
    Set {
        #[arg(value_name = "KEY")]
        key: String,
        #[arg(value_name = "VALUE")]
        value: String,
    },
    // Get value by key
    Get {
        #[arg(value_name = "KEY")]
        key: String,
    },
    // Remove value by key
    #[command(name = "rm")]
    Remove {
        #[arg(value_name = "KEY")]
        key: String,
    },
}
use std::process;

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Cli::parse();
    let mut kv = KvStore::open(Path::new("."))?;

    match args.commands {
        Commands::Set { key, value } => {
            kv.set(key, value)?;
        }
        Commands::Get { key } => match kv.get(key) {
            Ok(Some(value)) => println!("{}", value),
            Err(err) => println!("{}", err),
            Ok(None) => println!("Key not found"),
        },
        Commands::Remove { key } => match kv.remove(key) {
            Ok(_) => {}
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
        },
    }
    Ok(())
}
