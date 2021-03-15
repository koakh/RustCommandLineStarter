use std::process::Command;

// Struct std::process::Command
// https://doc.rust-lang.org/std/process/struct.Command.html
// https://doc.rust-lang.org/rust-by-example/std_misc/process.html
// https://rust-lang-nursery.github.io/rust-cookbook/os/external.html
// The process::Output struct represents the output of a finished child process, and the process::Command struct is a process builder.

pub fn execute_command(args: &[String]) -> Vec<u8> {
  let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(args)
      .output()
      .expect("failed to execute process")
  } else {
    Command::new("sh")
      .args(args)
      .output()
      .expect("failed to execute process")
  };

  output.stdout
}
