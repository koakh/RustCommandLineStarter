use std::fs::File;
// bulk import of many common io traits. like `reader.read_line(&mut buffer).unwrap()`
#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::io::{self, BufReader, Write};

pub fn search_content(pattern: &str, path: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
  let f = File::open(path);
  let content = match f {
    Ok(content) => content,
    Err(error) => {
      // used with Box<dyn std::error::Error>
      return Err(error.into());
    }
  };
  // printing performance
  let f = BufReader::new(content);
  for line in f.lines() {
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