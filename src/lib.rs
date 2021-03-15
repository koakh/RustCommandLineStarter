// in Rust, it is common to structure programs around the data they handle
#[derive(Debug)]
pub struct Cli {
  pub pattern: String,
  pub path: std::path::PathBuf,
}

pub fn greeting() {
  println!("Hello, world!");
}
