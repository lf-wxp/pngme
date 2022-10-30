use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
  let cli = args::Cli::parse();
  match cli.command {
    args::Commands::Encode(args) => {
      commands::encode(&args)?;
    },
    args::Commands::Decode(args) => {
      match commands::decode(&args) {
        Some(msg) => {
          println!("The message in chunk {:} is [{:}]" , args.chunk_type, msg)
        },
        None => println!("This is no message for chunk {:}", args.chunk_type)
      }
       
    },
    args::Commands::Remove(args) => {
      commands::remove(&args)?;
    },
    args::Commands::Print(args) => {
      commands::print_chunks(&args)?;
    },
  };
  Ok(())
}
