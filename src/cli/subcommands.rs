use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
    /// Synchronize remote repository data
    Sync,

    /// Update the system
    Update,

    /// List configured repositories
    ListRepos,

    /// Add an additional repository
    AddRepo { repo: String },

    /// Show information about <package>
    Info { pkg: String },

    /// Show file-list of <package>
    FileList { pkg: String },

    /// Show dependencies for <package>
    Deps { pkg: String },

    /// Show reverse dependendies of <package> (see man xbps-query)
    Reverse { pkg: String },

    /// Search for package by <name>
    Search { term: String },

    /// Search for package containing <file> (local)
    SearchFile { file: String },

    /// List installed packages
    List,

    /// Install <package(s)>
    Install { pkgs: Vec<String> },

    /// Install <package> (and corresponding <package>-devel package(s))
    DevInstall { pkgs: Vec<String> },

    /// List alternative candidates
    ListAlternatives { pkg: String },

    /// Set alternative for <package>
    SetAlternative { pkgs: Vec<String> },

    /// Re-configure installed <package>
    Reconfigure { pkg: String },

    /// Force installation of <package(s)>
    ForceInstall { pkgs: Vec<String> },

    /// Remove <package(s)> from the system
    Remove { pkgs: Vec<String> },

    /// Recursively remove package(s) (and its dependencies)
    RemoveRecursive { pkgs: Vec<String> },

    /// Clean up cache directory
    Cleanup,

    /// Remove orphaned packages
    AutoRemove,

    /// Search for package containing the file
    WhatProvides { file: PathBuf },
}
