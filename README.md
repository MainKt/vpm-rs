# vpm-rs
A Rust rewrite of vpm - An XBPS package management helper for Void Linux

### Building
You'll need to grab a
[Rust installation](https://www.rust-lang.org/) in order to compile it.

```shell
$ git clone https://github.com/MainKt/vpm-rs
$ cd vpm-rs
$ cargo build --release
$ ./target/release/vpm --version
vpm 0.1.0
```

### Usage
```
$ vpm
vpm-rs - An XBPS package management helper

Usage: vpm <COMMAND>

Commands:
  sync               Synchronize remote repository data
  update             Update the system
  list-repos         List configured repositories
  add-repo           Add an additional repository
  info               Show information about <package>
  file-list          Show file-list of <package>
  deps               Show dependencies for <package>
  reverse            Show reverse dependendies of <package> (see man xbps-query)
  search             Search for package by <name>
  search-file        Search for package containing <file> (local)
  list               List installed packages
  install            Install <package(s)>
  dev-install        Install <package> (and corresponding <package>-devel package(s))
  list-alternatives  List alternative candidates
  set-alternative    Set alternative for <package>
  reconfigure        Re-configure installed <package>
  force-install      Force installation of <package(s)>
  remove             Remove <package(s)> from the system
  remove-recursive   Recursively remove package(s) (and its dependencies)
  cleanup            Clean up cache directory
  auto-remove        Remove orphaned packages
  what-provides      Search for package containing the file
  help               Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
