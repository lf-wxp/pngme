use crate::{Error, Result};
use std::{convert::TryFrom, error, fmt, str::FromStr};

const CONDITION: u8 = 1 << 5;
/// Chunk type errors
#[derive(Debug)]
pub enum ChunkTypeError {
  /// Chunk has incorrect number of bytes (4 expected)
  ByteLengthError(usize),

  /// The input string contains an invalid character at the given index
  InvalidCharacter,
}

impl error::Error for ChunkTypeError {}
impl fmt::Display for ChunkTypeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ChunkTypeError::ByteLengthError(actual) => write!(
        f,
        "Expected 4 bytes but received {} when creating chunk type",
        actual
      ),
      ChunkTypeError::InvalidCharacter => {
        write!(f, "Input contains one or more invalid characters")
      }
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct ChunkType {
  bytes: [u8; 4],
}

impl ChunkType {
  fn is_valid_source(bytes: &[u8; 4]) -> bool {
    bytes
      .iter()
      .all(|&item| (65..=90).contains(&item) || (97..=122).contains(&item))
  }

  pub fn bytes(&self) -> [u8; 4] {
    self.bytes
  }
  fn is_critical(&self) -> bool {
    self.bytes[0] & CONDITION == 0
  }
  fn is_public(&self) -> bool {
    self.bytes[1] & CONDITION == 0
  }
  fn is_reserved_bit_valid(&self) -> bool {
    self.bytes[2] & CONDITION == 0
  }
  fn is_safe_to_copy(&self) -> bool {
    self.bytes[3] & CONDITION != 0
  }

  fn is_valid(&self) -> bool {
    return self.is_reserved_bit_valid();
  }
}

impl TryFrom<[u8; 4]> for ChunkType {
  type Error = ChunkTypeError;
  fn try_from(bytes: [u8; 4]) -> std::result::Result<ChunkType, ChunkTypeError> {
    if ChunkType::is_valid_source(&bytes) {
      return Ok(ChunkType { bytes });
    }
    Err(ChunkTypeError::InvalidCharacter)
  }
}

impl FromStr for ChunkType {
  type Err = Error;
  fn from_str(str: &str) -> Result<ChunkType> {
    let bytes: Vec<u8> = String::from(str).bytes().collect();

    if bytes.len() != 4 {
      return Err(Box::new(ChunkTypeError::ByteLengthError(bytes.len())));
    }

    let is_invalid_char = !ChunkType::is_valid_source(bytes[..].try_into().unwrap());

    if is_invalid_char {
      return Err(Box::new(ChunkTypeError::InvalidCharacter));
    }
    Ok(ChunkType {
      bytes: bytes[..].try_into().unwrap(),
    })
  }
}
impl fmt::Display for ChunkType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", String::from_utf8(self.bytes.to_vec()).unwrap())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::convert::TryFrom;
  use std::str::FromStr;

  #[test]
  pub fn test_chunk_type_from_bytes() {
    let expected = [82, 117, 83, 116];
    let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

    assert_eq!(expected, actual.bytes());
  }

  #[test]
  pub fn test_chunk_type_from_str() {
    let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
    let actual = ChunkType::from_str("RuSt").unwrap();
    assert_eq!(expected, actual);
  }

  #[test]
  pub fn test_chunk_type_is_critical() {
    let chunk = ChunkType::from_str("RuSt").unwrap();
    assert!(chunk.is_critical());
  }

  #[test]
  pub fn test_chunk_type_is_not_critical() {
    let chunk = ChunkType::from_str("ruSt").unwrap();
    assert!(!chunk.is_critical());
  }

  #[test]
  pub fn test_chunk_type_is_public() {
    let chunk = ChunkType::from_str("RUSt").unwrap();
    assert!(chunk.is_public());
  }

  #[test]
  pub fn test_chunk_type_is_not_public() {
    let chunk = ChunkType::from_str("RuSt").unwrap();
    assert!(!chunk.is_public());
  }

  #[test]
  pub fn test_chunk_type_is_reserved_bit_valid() {
    let chunk = ChunkType::from_str("RuSt").unwrap();
    assert!(chunk.is_reserved_bit_valid());
  }

  #[test]
  pub fn test_chunk_type_is_reserved_bit_invalid() {
    let chunk = ChunkType::from_str("Rust").unwrap();
    assert!(!chunk.is_reserved_bit_valid());
  }

  #[test]
  pub fn test_chunk_type_is_safe_to_copy() {
    let chunk = ChunkType::from_str("RuSt").unwrap();
    assert!(chunk.is_safe_to_copy());
  }

  #[test]
  pub fn test_chunk_type_is_unsafe_to_copy() {
    let chunk = ChunkType::from_str("RuST").unwrap();
    assert!(!chunk.is_safe_to_copy());
  }

  #[test]
  pub fn test_valid_chunk_is_valid() {
    let chunk = ChunkType::from_str("RuSt").unwrap();
    assert!(chunk.is_valid());
  }

  #[test]
  pub fn test_invalid_chunk_is_valid() {
    let chunk = ChunkType::from_str("Rust").unwrap();
    assert!(!chunk.is_valid());

    let chunk = ChunkType::from_str("Ru1t");
    assert!(chunk.is_err());
  }

  #[test]
  pub fn test_chunk_type_string() {
    let chunk = ChunkType::from_str("RuSt").unwrap();
    assert_eq!(&chunk.to_string(), "RuSt");
  }

  #[test]
  pub fn test_chunk_type_trait_impls() {
    let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
    let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
    let _chunk_string = format!("{}", chunk_type_1);
    let _are_chunks_equal = chunk_type_1 == chunk_type_2;
  }
}
