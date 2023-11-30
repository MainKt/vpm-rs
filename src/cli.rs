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
            Commands::Info { pkg } => {
                println!("[vpm] Info on {pkg} (xbps-query -v -R {pkg}):");
                Command::new("xbps-query")
                    .args(&["-v", "-R"])
                    .arg(pkg)
                    .spawn()?
                    .wait()
            }
            Commands::FileList { pkg: _ } => todo!(),
            Commands::Deps { pkg: _ } => todo!(),
            Commands::Reverse { pkg: _ } => todo!(),
            Commands::Search { term } => {
                println!("Searching for {term}, (xbps-query -v -Rs {term}):");
                Command::new("xbps-query")
                    .args(&["-v", "-Rs"])
                    .arg(term)
                    .spawn()?
                    .wait()
            }
            Commands::SearchFile { file } => {
                println!("Searching for {file}, (xbps-query -v -Rs \"*/{file}\"):");
                Command::new("xbps-query")
                    .args(&["-v", "-o"])
                    .arg(format!("\"*/{file}\""))
                    .spawn()?
                    .wait()
            }
            Commands::List => {
                println!("[vpm] Installed packages (xbps-query -v -l): ");
                Command::new("xbps-query")
                    .arg("-v")
                    .arg("-l")
                    .spawn()?
                    .wait()
            }
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
            Commands::Remove { pkgs } => {
                let pkg_names = pkgs.join(" ");
                println!("[vpm] Removing packages [{pkg_names}], (xbps-remove -v {pkg_names}):");
                Command::new("xbps-remove")
                    .arg("-v")
                    .args(pkgs)
                    .spawn()?
                    .wait()
            }
            Commands::RemoveRecursive { pkgs } => {
                let pkg_names = pkgs.join(" ");
                println!("[vpm] Removing package(s) recursively [{pkg_names}], (xbps-remove -v -R {pkg_names}):");
                Command::new("xbps-remove")
                    .args(&["-v", "-R"])
                    .args(pkgs)
                    .spawn()?
                    .wait()
            }
            Commands::Cleanup => {
                println!("[vpm] Cleaning up packages (will remove orphaned packages) (xbps-remove -v -O clean):");
                Command::new("xbps-remove")
                    .args(&["-v", "-O"])
                    .arg("clean")
                    .spawn()?
                    .wait()
            }
            Commands::AutoRemove => {
                println!("[vpm] Removing orphaned packages (xbps-install -v -o):");
                Command::new("xbps-remove")
                    .args(&["-v", "-o"])
                    .spawn()?
                    .wait()
            }
            Commands::WhatProvides { file } => {
                println!(
                    "[vpm]  relaying to: `xlocate foo` - use xlocate -S to (re-)build cached DB."
                );
                let child = Command::new("xlocate").arg(file).spawn();
                if let Err(ref e) = child {
                    if e.kind() == io::ErrorKind::NotFound {
                        eprintln!("xlocate not found. Try installing the xtools package.");
                        std::process::exit(1);
                    }
                }
                child?.wait()
            }
        }
    }
}
