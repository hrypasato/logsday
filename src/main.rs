mod model;
mod cli;
mod storage;

use clap::{Parser};
use std::{error::Error};
use crate::cli::Cli;
use crate::model::Entry;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let entry = Entry::new(cli.task, cli.level, cli.source);
    println!("Done");
    storage::save(entry)
}
