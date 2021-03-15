#![allow(unused_imports)]
use command_line_apps_in_rust::{greeting, Cli};

fn main() {
  // no used variable, prefixed with _
  let pattern = std::env::args().nth(1).expect("no pattern given");
  let path = std::env::args().nth(2).expect("no path given");
  // same with match option
  // let path_option = std::env::args().nth(2);
  // let path = match path_option {
  //   Some(v) => v,
  //   None => panic!("no path"),
  // };
  let args = Cli {
    pattern,
    path: std::path::PathBuf::from(path),
  };
  println!("{:?}", args);
}
