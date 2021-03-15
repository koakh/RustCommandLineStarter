// #![allow(unused_imports)]
use command_line_apps_in_rust::{Cli};
use structopt::StructOpt;

fn main() {
  let args = Cli::from_args();
  println!("{:?}", args);
}
