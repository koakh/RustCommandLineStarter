#![allow(unused_imports)]
// main lib.rs
use command_line_apps_in_rust::search_content;
// use/init modules
mod constants;
mod http_client;
mod process;
mod subcommands;
// bring data module used data structures
// self says we’re finding a module that’s a child of the current module (is optional, work without self:: to)
use self::constants::API_KEY;
use self::http_client::{
  async_request_generic_typed, async_request_untyped, block_request, HttpBinResponse,
};
use self::process::execute_command;
use self::subcommands::Cli;

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
  let cli = Cli::from_args();
  // show args
  debug!("{:?}", cli);
  debug!("apiKey: {}", API_KEY);

  match &cli {
    // test with: `cargo run -- search -f "Loading snapper configs" -p logs/snapper.log`
    Cli::Search { pattern, path } => {
      println!("{:?} {:?}", &pattern, &path);
      search_content(&pattern, &path)
    }
    // test with: `cargo run -- command --command "ls -la ~"`
    Cli::Command { command } => {
      test_execute_command(command);
      Ok(())
    }
    // test with: `cargo run -- req-untyped`
    Cli::AsyncRequestUntyped {} => {
      test_async_request_untyped().await?;
      Ok(())
    }
    // test with: `cargo run -- req-typed`
    Cli::AsyncRequestGenericTyped {} => {
      test_async_request_generic_typed().await?;
      Ok(())
    }
    Cli::Add {
      interactive,
      patch,
      files,
    } => {
      println!("{:?} {:?} {:?}", interactive, patch, files);
      unimplemented!()
    }
    Cli::Fetch {
      dry_run,
      all,
      repository,
    } => {
      println!("{:?} {:?} {:?}", dry_run, all, repository);
      unimplemented!()
    }
    Cli::Commit { message, all } => {
      println!("{:?} {:?}", message, all);
      unimplemented!()
    }
  }

  // match &cli {
  //   Cli::Search(pattern, path) => {
  //     search_content(&pattern, &path)
  //   }
  //   Cli::Add { _, _, _ } => {  }
  //   Cli::Fetch { _, _, _ } => {  }
  //   Cli::Commit { _, _, _ } => {  }
  // }

  // test: execute_command
  // test_execute_command();

  // test: block request, don't use at same time with tokio
  // else `thread 'main' panicked at 'Cannot drop a runtime in a context where blocking is not allowed. This happens when a runtime is dropped from within an asynchronous context.'`
  // test_block_request();

  // test: operating on untyped JSON values
  // test_async_request_untyped().await?;

  // test: parsing JSON as strongly typed data structures, and with generics
  // test_async_request_generic_typed().await?;
}

fn test_execute_command(command: &str) {
  // let command_args = &[String::from("-c"), String::from("ls -lah /tmp")];
  let command_args = &[String::from("-c"), String::from(command)];
  println!("{:?}", command_args);
  let bytes = execute_command(command_args);
  let s = String::from_utf8(bytes).expect("invalid UTF-8");
  print!("{}", s);
}

#[allow(dead_code)]
fn test_block_request() {
  // test: block request, don't use at same time with tokio
  let response = block_request();
  debug!("response: {:?}", response);
}

#[allow(dead_code)]
async fn test_async_request_untyped() -> Result<(), Box<dyn std::error::Error>> {
  // test: operating on untyped JSON values
  let valued_untyped = async_request_untyped().await;
  match valued_untyped {
    Ok(valued_untyped) => {
      println!(
        "valued_untyped: please call {} at the number {}",
        valued_untyped["name"], valued_untyped["phones"][0]
      );
      Ok(())
    }
    Err(error) => {
      // propagate error
      Err(error)
    }
  }
}

async fn test_async_request_generic_typed() -> Result<(), Box<dyn std::error::Error>> {
  // test: parsing JSON as strongly typed data structures, and with generics
  let value_typed: Result<Box<HttpBinResponse>, Box<dyn std::error::Error>> =
    async_request_generic_typed().await;
  match value_typed {
    Ok(value) => {
      println!("value_typed: called from origin: {}", value.origin);
      Ok(())
    }
    Err(error) => Err(error),
  }
}
