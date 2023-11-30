use clap::Parser;
use std::{io, process};

mod cli;

fn main() -> io::Result<()> {
    let cli = cli::Cli::parse();

    if let Some(code) = cli.execute()?.code() {
        process::exit(code);
    }

    Ok(())
}
