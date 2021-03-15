// #[allow(unused_imports)]
use command_line_apps_in_rust::{search_content, Cli};
// env_logger
#[allow(unused_imports)]
use log::{info, warn};
use structopt::StructOpt;

// main required io::Result<()> to prevent cannot use the `?` operator in a function that returns `()`
// in File::open("foo.txt")?
// fn main() -> io::Result<()> {
// Box<dyn std::error::Error> is also an interesting type. It’s a Box that can contain any type that implements the standard Error trait. This means that basically all errors can be put into this box, so we can use ? on all of the usual functions that return Results.
fn main() -> Result<(), Box<dyn std::error::Error>> {
  // or use `env RUST_LOG=output_log=info cargo run -- flags`
  std::env::set_var("RUST_LOG", "output_log=info");
  // init env logger before anything else and in main
  env_logger::init();
  // the from_args method is meant to be used in your main function.
  // When it fails, it will print out an error or help message and immediately exit the program. Don’t use it in other places!
  let args = Cli::from_args();
  info!("{:?}", args);

  search_content(&args.pattern, &args.path)
}
