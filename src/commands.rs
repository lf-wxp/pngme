use std::convert::TryFrom;
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
  let bytes = fs::read(&file_path)?;
  let mut png = Png::try_from(&bytes[..])?;
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
  let bytes = fs::read(&file_path).ok()?;
  let png = Png::try_from(bytes.as_slice()).ok()?;
  let chunk = png.chunk_by_type(chunk_type.as_str())?;
  chunk.data_as_string().ok()
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: &RemoveArgs) -> Result<()> {
  let RemoveArgs { file_path, chunk_type } = args;
  let bytes = fs::read(&file_path)?;
  let mut png = Png::try_from(bytes.as_slice())?;
  png.remove_chunk(chunk_type)?;
  fs::write(file_path, png.as_bytes())?;
  Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: &PrintArgs) -> Result<()> {
  let PrintArgs { file_path  } = args;
  let bytes = fs::read(&file_path)?;
  let png = Png::try_from(bytes.as_slice())?;
  let chunks = png.chunks();
  println!("the chunks length is {}", chunks.len());
  for chunk in png.chunks() {
    if !chunk.chunk_type().is_critical() {
      println!("{}", chunk)
    }
  }
  Ok(())
}
