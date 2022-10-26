use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  Encode(EncodeArgs),
  Decode(DecodeArgs),
  Remove(RemoveArgs),
  Print(PrintArgs),
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct EncodeArgs {
  #[clap(value_parser)]
  file_path: PathBuf,
  #[clap(value_parser)]
  chunk_type: String,
  #[clap(value_parser)]
  message: String,
  #[clap(value_parser)]
  output: Option<PathBuf>,
}
#[derive(Args, Debug)]
pub struct DecodeArgs {
  #[clap(value_parser)]
  file_path: PathBuf,
  #[clap(value_parser)]
  chunk_type: String,
}
#[derive(Args, Debug)]
pub struct RemoveArgs {
  #[clap(value_parser)]
  file_path: PathBuf,
  #[clap(value_parser)]
  chunk_type: String,
}
#[derive(Args, Debug)]
pub struct PrintArgs {
  #[clap(value_parser)]
  file_path: PathBuf,
}
