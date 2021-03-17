use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;
// bulk import of many common io traits. like `reader.read_line(&mut buffer).unwrap()`
#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::io::{self, BufReader, Write};

// // old struct without subcommands : struct
// // in Rust, it is common to structure programs around the data they handle
// #[derive(Debug, StructOpt)]
// pub struct Cli {
//   /// the pattern to look for
//   #[structopt(short = "f", long = "pattern")]
//   pub pattern: String,
//   /// the path to the file to read
//   #[structopt(short = "p", long = "path")]
//   #[structopt(parse(from_os_str))]
//   pub path: std::path::PathBuf,
// }

#[derive(Debug, StructOpt)]
#[structopt(
  name = "command-line-apps-in-rust",
  about = "koakh rust command line starter"
)]
pub enum Cli {
  /// search pattern in file
  #[structopt(name = "search")]
  Search {
    /// the pattern to look for
    #[structopt(short = "f", long = "pattern")]
    pattern: String,
    /// the path to the file to read
    #[structopt(short = "p", long = "path")]
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
  },
  /// execute a cross os command
  #[structopt(name = "command")]
  Command {
    /// the command to execute
    #[structopt(short = "c", long = "command")]
    command: String,
  },
  /// block request
  // #[structopt(name = "block_request")]
  // BlockRequest {
  // },
  /// untyped request, use serde value
  #[structopt(name = "req-untyped")]
  AsyncRequestUntyped {},
  /// untyped request, use serde value
  #[structopt(name = "req-typed")]
  AsyncRequestGenericTyped {},
  /// git add
  #[structopt(name = "add")]
  Add {
    #[structopt(short = "i", long = "interactive")]
    interactive: bool,
    #[structopt(short = "p", long = "patch")]
    patch: bool,
    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
  },
  /// git fetch
  #[structopt(name = "fetch")]
  Fetch {
    #[structopt(long = "dry-run")]
    dry_run: bool,
    #[structopt(long = "all")]
    all: bool,
    repository: Option<String>,
  },
  /// git commit
  #[structopt(name = "commit")]
  Commit {
    #[structopt(short = "m", long = "message")]
    message: Option<String>,
    #[structopt(short = "a", long = "all")]
    all: bool,
  },
}
