use std::fs;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::{Chunk};
use crate::chunk_type::{ChunkType};
use crate::png::Png;
use crate::{Result};

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: &EncodeArgs) -> Result<()> {
  let EncodeArgs { file_path, chunk_type, message, output } = args;
  let mut png = Png::from_file(file_path.to_path_buf())?;
  let chunk_type = ChunkType::from_str(chunk_type.as_str())?;
  let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());
  png.append_chunk(chunk);
  let path = match output {
    Some(path) => path,
    None => file_path, 
  };
  fs::write(path, png.as_bytes())?;
  Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: &DecodeArgs) -> Option<String> {
  let DecodeArgs { file_path, chunk_type } = args;
  let png = Png::from_file(file_path.to_path_buf()).ok()?;
  let chunk = png.chunk_by_type(chunk_type.as_str())?;
  chunk.data_as_string().ok()
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: &RemoveArgs) -> Result<()> {
  let RemoveArgs { file_path, chunk_type } = args;
  let mut png = Png::from_file(file_path.to_path_buf())?;
  png.remove_chunk(chunk_type)?;
  fs::write(file_path, png.as_bytes())?;
  Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: &PrintArgs) -> Result<()> {
  let PrintArgs { file_path  } = args;
  let png = Png::from_file(file_path.to_path_buf())?;
  for chunk in png.chunks() {
    if let Ok(msg) = chunk.data_as_string() {
      if !msg.trim().is_empty() {
        println!("the chunk type is {}, the msg is {}" , chunk.chunk_type(), msg);
      }
    }
  }
  Ok(())
}
