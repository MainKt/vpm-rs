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
            Commands::FileList { pkg } => {
                println!("[vpm] file-list of {pkg} (xbps-query -v -R -f {pkg}):");
                Command::new("xbps-query")
                    .args(&["-v", "-R", "-f"])
                    .arg(pkg)
                    .spawn()?
                    .wait()
            }
            Commands::Deps { pkg } => {
                println!("[vpm] Dependencies for {pkg} (xbps-query -v -R -x {pkg}):");
                Command::new("xbps-query")
                    .args(&["-v", "-R", "-x"])
                    .arg(pkg)
                    .spawn()?
                    .wait()
            }
            Commands::Reverse { pkg } => {
                println!("[vpm] Reverse dependencies for {pkg}, (xbps-query -v -R -X {pkg}):");
                Command::new("xbps-query")
                    .args(&["-v", "-R", "-X"])
                    .arg(pkg)
                    .spawn()?
                    .wait()
            }
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
            Commands::DevInstall { pkgs } => {
                println!("[vpm] Packages will be installed one-by-one. Use `force-install` to override this if you know what you're doing.");
                println!("(Note: `force-install` won't install -devel packages)");

                let mut exit_status = ExitStatus::default();
                for pkg in pkgs {
                    println!("Installing {pkg}, (xbps-install -S {pkg}):");
                    let exit_status_pkg = Command::new("xbps-install")
                        .arg("-S")
                        .arg(&pkg)
                        .spawn()?
                        .wait()?;
                    if exit_status_pkg != ExitStatus::default() {
                        exit_status = exit_status_pkg;
                        break;
                    }

                    println!("Installing devel package for {pkg}, (xbps-install -S {pkg}-devel):");
                    let exit_status_devel = Command::new("xbps-install")
                        .arg("-S")
                        .arg(format!("{pkg}-devel"))
                        .spawn()?
                        .wait()?;
                    if exit_status_devel != ExitStatus::default() {
                        exit_status = exit_status_devel;
                        break;
                    }
                }
                Ok(exit_status)
            }
            Commands::ListAlternatives { pkg } => {
                println!("[vpm] Alternatives for {pkg}, (xbps-alternatives -l {pkg}):");
                Command::new("xbps-alternatives")
                    .arg("-l")
                    .arg(pkg)
                    .spawn()?
                    .wait()
            }
            Commands::SetAlternative { pkgs } => {
                let pkg_names = pkgs.join(" ");
                println!("[vpm] Setting alternative for {pkg_names}, (xbps-alternatives -s {pkg_names}):");
                Command::new("xbps-alternatives")
                    .arg("-s")
                    .args(pkgs)
                    .spawn()?
                    .wait()
            }
            Commands::Reconfigure { pkg } => {
                println!("[vpm] Re-configuring {pkg}, (xbps-reconfigure -v {pkg})");
                Command::new("xbps-reconfigure")
                    .arg("-v")
                    .arg(pkg)
                    .spawn()?
                    .wait()
            }
            Commands::ForceInstall { pkgs } => {
                let pkg_names = pkgs.join(" ");
                println!("[vpm] Force-Installing [{pkg_names}], (xbps-install -Sf {pkg_names})");
                Command::new("xbps-install")
                    .arg("-Sf")
                    .args(pkgs)
                    .spawn()?
                    .wait()
            }
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
