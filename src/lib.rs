use structopt::StructOpt;
use std::fs::File;
// bulk import of many common io traits. like `reader.read_line(&mut buffer).unwrap()`
#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::io::{self, BufReader, Write};

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

pub fn search_content(pattern: &str, path: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>>{
    // using read_to_string
  // let content = std::fs::read_to_string(&args.path).expect("could not read the file");
  // for line in content.lines() {
  //   if line.contains(&args.pattern) {
  //     println!("{}", line);
  //   }
  // }

  // Exercise for the reader: This is not the best implementation: It will read the whole file into memory
  // however large the file may be. Find a way to optimize it! (One idea might be to use a BufReader instead of read_to_string()
  // The BufReader<R> struct adds buffering to any reader.
  // https://gist.github.com/ccdle12/48ec24f4e25b3f289b873a1d32b41980#bufwriter-and-bufreader
  // opt we can use ? to panic with a -> io::Result<()>
  // let f = File::open(&args.path)?;

  // or we can `No need to panic` and work witch exception
  let f = File::open(path);
  let content = match f {
    Ok(content) => content,
    Err(error) => {
      // used with Box<dyn std::error::Error>
      return Err(error.into());
    }
  };
  // snippet to read first line
  // let mut reader = BufReader::new(f);
  // let mut buffer = String::new();
  // reader.read_line(&mut buffer).unwrap();

  // code to loop buffer reader lines
  // https://doc.rust-lang.org/std/io/trait.BufRead.html
  // https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
  //
  // printing performance
  let f = BufReader::new(content);
  for line in f.lines() {
    // without writeln to BufWriter handle
    // println!("{}", line.unwrap());
    // printing to the terminal is surprisingly slow! If you call things like println! in a loop, it can easily become a bottleneck in an otherwise fast program. To speed this up, there are two things you can do.
    // https://rust-cli.github.io/book/tutorial/output.html#a-note-on-printing-performance
    // get the global stdout entity
    let stdout = io::stdout();
    // optional: wrap that handle in a buffer
    // wrap stdout handle in a BufWriter which by default buffers up to 8 kB
    let mut handle = io::BufWriter::new(stdout);
    let line_content = line.unwrap();
    if line_content.contains(pattern) {
      // add `?` if you care about errors here
      writeln!(handle, "{}", line_content)?;
    }
  }
  Ok(())
}