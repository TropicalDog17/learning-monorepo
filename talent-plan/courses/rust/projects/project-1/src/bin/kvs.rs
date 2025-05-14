use std::collections::HashMap;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

pub struct KvStore {
    kv: HashMap<String, String>,
}

impl KvStore {
    fn new() -> Self {
        KvStore {
            kv: HashMap::<String, String>::new(),
        }
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.kv.get(key)
    }

    fn set(&mut self, key: String, value: String) -> Option<String> {
        self.kv.insert(key, value)
    }

    fn remove(&mut self, key: &str) -> Option<(String, String)> {
        self.kv.remove_entry(key)
    }
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
        is_true: Option<bool>,
    },
    Rm {
        key: String,
    },
}
fn main() -> Result<()> {
    let mut kv = KvStore::new();

    let args = Args::parse();

    match args.cmd {
        Commands::Get { key } => {
            // let value = kv.get(&key).expect("Key not exists");
            // println!("value is: {}", value);
            unimplemented!("unimplemented")
        }
        Commands::Set {
            key,
            value,
            is_true,
        } => {
            // let entry = kv.get(&key);
            // match entry {
            //     None => {
            //         kv.set(key.clone(), value.clone());
            //     }
            //     Some(_) => {
            //         if is_true {
            //             kv.set(key.clone(), value.clone());
            //         }
            //     }
            // }
            // println!("Inserted key: {} with value: {}", key, value);

            unimplemented!("unimplemented")
        }
        Commands::Rm { key } => {
            // kv.remove(&key).expect("Key not exists");
            unimplemented!("unimplemented")
        }
    }
    Ok(())
}
