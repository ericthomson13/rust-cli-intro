use structopt::StructOpt;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
  /// The pattern to look for
  pattern: String,
  /// The path to the file to read
  #[structopt(parse(from_os_str))]
  path: std::path::PathBuf,
}
/// Custom error type that allows us to build custom error messages
#[derive(Debug)]
struct CustomError(String);

use anyhow::{Context, Result};

fn main () -> Result<()> {
  let path = "text.txt";
  let content = std::fs::read_to_string(path).with_context(|| format!("could not read file `{}`", path))?;
  println!("file content: {}", content);
  Ok(())
}
// #![allow(unused)]
// fn main() {
//   let content = std::fs::read_t0o_string("test.txt").unwrap();
//   println!("file content: {}", content);
// }
// fn main() {
//   let args = Cli::from_args();
//   let content = std::fs::read_to_string(&args.path).expect("could not read file");
//   for line in content.lines() {
//     if line.contains(&args.pattern) {
//       println!("{}", line);
//     }
//   }
// }