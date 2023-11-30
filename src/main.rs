use clap::Parser;

mod cli;

fn main() {
    let cli = cli::Cli::parse();
    cli.execute();
}
