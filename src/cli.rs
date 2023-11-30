use clap::Parser;
use std::process::Command;
use std::{io, process::ExitStatus};

mod subcommands;

use subcommands::Commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn execute(self) -> io::Result<ExitStatus> {
        match self.command {
            Commands::Sync => {
                println!("[vpm] Synchronising remote repository data (xbps-install -S):");
                Command::new("xbps-install").arg("-S").spawn()?.wait()
            }
            Commands::Update => {
                println!("[vpm] Running system update (xbps-install -Suv):");
                Command::new("xbps-install").arg("-Suv").spawn()?.wait()
            }
            Commands::ListRepos => {
                println!("[vpm] Configured repositories (xbps-query -v -L):");
                Command::new("xbps-query").arg("-vL").spawn()?.wait()?;

                println!("[vpm] Available sub-repositories (xbps-query -v -Rs void-repo):");
                Command::new("xbps-query")
                    .arg("-v")
                    .arg("-Rs")
                    .arg("void-repo")
                    .spawn()?
                    .wait()
            }
            Commands::AddRepo { repos: _ } => todo!(),
            Commands::Info { pkg: _ } => todo!(),
            Commands::FileList { pkg: _ } => todo!(),
            Commands::Deps { pkg: _ } => todo!(),
            Commands::Reverse { pkg: _ } => todo!(),
            Commands::Search { term: _ } => todo!(),
            Commands::SearchFile { file: _ } => todo!(),
            Commands::List => todo!(),
            Commands::Install { pkgs } => {
                let pkg_names = pkgs.join(" ");
                println!("[vpm] Installing packages [{pkg_names}]: (xbps-install -S {pkg_names})");
                Command::new("xbps-install")
                    .arg("-S")
                    .args(pkgs)
                    .spawn()?
                    .wait()
            }
            Commands::DevInstall { pkgs: _ } => todo!(),
            Commands::ListAlternatives => todo!(),
            Commands::SetAlternative { pkgs: _ } => todo!(),
            Commands::Reconfigure { pkg: _ } => todo!(),
            Commands::ForceInstall { pkgs: _ } => todo!(),
            Commands::Remove { pkgs: _ } => todo!(),
            Commands::RemoveRecursive { pkgs: _ } => todo!(),
            Commands::Cleanup => todo!(),
            Commands::AutoRemove => todo!(),
            Commands::WhatProvides { file: _ } => todo!(),
        }
    }
}
