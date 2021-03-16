#![allow(unused_imports)]
// main lib.rs
use command_line_apps_in_rust::{search_content, Cli};
// use/init modules
mod constants;
mod http_client;
mod process;
// bring data module used data structures
// self says we’re finding a module that’s a child of the current module (is optional, work without self:: to)
use self::constants::API_KEY;
use self::http_client::{HttpBinResponse, async_request, async_request_untyped, block_request};
use self::process::execute_command;

// third party

// env_logger
use log::{debug, error, info, trace, warn};
// structopt
use structopt::StructOpt;
// serde
use serde_json::Value;

// Box<dyn std::error::Error> is also an interesting type. It’s a Box that can contain any type that implements the standard Error trait. This means that basically all errors can be put into this box, so we can use ? on all of the usual functions that return Results.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // or use `env RUST_LOG=output_log=info cargo run -- flags`
  std::env::set_var("RUST_LOG", "output_log=debug");
  // init env logger before anything else and in main
  env_logger::init();
  // the from_args method is meant to be used in your main function.
  // When it fails, it will print out an error or help message and immediately exit the program. Don’t use it in other places!
  let args = Cli::from_args();
  // show args
  debug!("{:?}", args);

  debug!("apiKey: {}", API_KEY);
  let command_args = &[String::from("-c"), String::from("ls -lah /tmp")];
  let bytes = execute_command(command_args);
  let s = String::from_utf8(bytes).expect("invalid UTF-8");
  print!("{}", s);

  // test: block request, don't use at same time with tokio
  // let response = block_request();
  // debug!("response: {:?}", response);

  // let res = reqwest::get("http://httpbin.org/get").await?;
  // println!("status: {}", res.status());
  // println!("headers:\n{:#?}", res.headers());
  // let body = res.text().await?;
  // println!("body:\n{}", body);

  // test: operating on untyped JSON values
  let valued_untyped = async_request_untyped().await;
  match valued_untyped {
    Ok(valued_untyped) => {
      println!(
        "valued_untyped: please call {} at the number {}",
        valued_untyped["name"], valued_untyped["phones"][0]
      )
    }
    Err(error) => {
      error!("{}", error);
    }
  }
  // Parsing JSON as strongly typed data structures, and with generics
  let value_typed: Result<Box<HttpBinResponse>,_> = async_request().await;
  match value_typed {
    Ok(value) => {
      println!("value_typed: called from origin: {}", value.origin)
    }
    Err(error) => {
      error!("{}", error);
    }
  }

  // return result
  search_content(&args.pattern, &args.path)
}
