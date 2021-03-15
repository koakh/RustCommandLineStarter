use structopt::StructOpt;

// in Rust, it is common to structure programs around the data they handle
#[derive(Debug)]
#[derive(StructOpt)]
pub struct Cli {
  /// the pattern to look for
  #[structopt(short = "f", long = "pattern")]
  pub pattern: String,
  /// the path to the file to read
  #[structopt(short = "p", long = "path")]
  #[structopt(parse(from_os_str))]
  pub path: std::path::PathBuf,
}

pub fn greeting() {
  println!("Hello, world!");
}


// pub fn search_content(f: io::Result<File>, )