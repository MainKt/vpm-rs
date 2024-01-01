use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
    /// Synchronize remote repository data
    Sync,

    /// Update the system
    #[command(alias = "up")]
    Update,

    /// List configured repositories
    #[command(alias = "lr")]
    ListRepos,

    /// Add an additional repository
    #[command(alias = "ar")]
    AddRepo { repo: String },

    /// Show information about <package>
    #[command(alias = "if")]
    Info { pkg: String },

    /// Show file-list of <package>
    #[command(alias = "fl")]
    FileList { pkg: String },

    /// Show dependencies for <package>
    Deps { pkg: String },

    /// Show reverse dependencies of <package> (see man xbps-query)
    #[command(alias = "rdeps")]
    ReverseDeps { pkg: String },

    /// Search for package by <name>
    #[command(alias = "se")]
    Search { term: String },

    /// Search for package containing <file> (local)
    #[command(alias = "sf")]
    SearchFile { file: String },

    /// List installed packages
    #[command(alias = "ls")]
    List,

    /// Install <package(s)>
    #[command(alias = "in")]
    Install { pkgs: Vec<String> },

    /// Install <package> (and corresponding <package>-devel package(s))
    #[command(alias = "dev")]
    DevInstall { pkgs: Vec<String> },

    /// List alternative candidates
    #[command(alias = "la")]
    ListAlternatives { pkg: String },

    /// Set alternative for <package>
    #[command(alias = "sa")]
    SetAlternative { pkgs: Vec<String> },

    /// Re-configure installed <package>
    Reconfigure { pkg: String },

    /// Force installation of <package(s)>
    #[command(alias = "fin")]
    ForceInstall { pkgs: Vec<String> },

    /// Remove <package(s)> from the system
    #[command(alias = "rm")]
    Remove { pkgs: Vec<String> },

    /// Recursively remove package(s) (and its dependencies)
    #[command(alias = "rmr")]
    RemoveRecursive { pkgs: Vec<String> },

    /// Clean up cache directory
    #[command(alias = "cl")]
    Cleanup,

    /// Remove orphaned packages
    #[command(alias = "arm")]
    AutoRemove,

    /// Search for package containing the file
    #[command(alias = "wp")]
    WhatProvides { file: PathBuf },
}
