use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
  Encode(EncodeArgs),
  Decode(DecodeArgs),
  Remove(RemoveArgs),
  Print(PrintArgs),
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct EncodeArgs {
  #[clap(value_parser)]
  pub file_path: PathBuf,
  #[clap(value_parser)]
  pub chunk_type: String,
  #[clap(value_parser)]
  pub message: String,
  #[clap(value_parser)]
  pub output: Option<PathBuf>,
}
#[derive(Args, Debug)]
pub struct DecodeArgs {
  #[clap(value_parser)]
  pub file_path: PathBuf,
  #[clap(value_parser)]
  pub chunk_type: String,
}
#[derive(Args, Debug)]
pub struct RemoveArgs {
  #[clap(value_parser)]
  pub file_path: PathBuf,
  #[clap(value_parser)]
  pub chunk_type: String,
}
#[derive(Args, Debug)]
pub struct PrintArgs {
  #[clap(value_parser)]
  pub file_path: PathBuf,
}
