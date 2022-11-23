use std::{str::FromStr, fmt::Display};

use crate::{Error, Result};


#[derive(Debug, PartialEq, Eq)]
struct ChunkType {
    chunk: [char; 4]
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.chunk.map(|c| c as u8)
    }

    fn is_valid(&self) -> bool {
        match self.chunk[2] {
            c if c.is_ascii_lowercase() => false,
            c if c.is_ascii_uppercase() => true,
            _ => panic!("is_valid encountered fatal error")
        }
    }

    fn is_critical(&self) -> bool {
        match self.chunk[0] {
            c if c.is_ascii_uppercase() => true,
            c if c.is_ascii_lowercase() => false,
            _ => panic!("is_critical encountered fatal error")
        }
    }
    
    fn is_public(&self) -> bool {
        match self.chunk[1] {
            c if c.is_ascii_uppercase() => true,
            c if c.is_ascii_lowercase() => false,
            _ => panic!("is_public encountered fatal error")
        }
    }
    
    fn is_reserved_bit_valid(&self) -> bool {
        match self.chunk[2] {
            c if c.is_ascii_uppercase() => true,
            c if c.is_ascii_lowercase() => false,
            _ => panic!("is_reserved_bit_valid encountered fatal error")
        }
    }
    
    fn is_safe_to_copy(&self) -> bool {
        match self.chunk[3] {
            c if c.is_ascii_uppercase() => false,
            c if c.is_ascii_lowercase() => true,
            _ => panic!("is_safe_to_copy encountered fatal error")
        }
    }

}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        let mut chunk = [' '; 4];
        for (i, val) in value.into_iter().enumerate() {
            match val {
               val if (val as char).is_ascii_alphabetic() => chunk[i] = val as char,
               _ => return Err("Invalid input format".into())
            }
        }
        Ok(ChunkType { chunk })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut chunk = [' '; 4];
        if s.len() > 4 {
            return Err("Invalid input format".into());
        }
        for (i, c) in s.chars().enumerate() {
            match c {
               c if c.is_ascii_alphabetic() => chunk[i] = c,
               _ => return Err("Invalid input format".into())
            }
        }
        Ok(ChunkType { chunk })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.chunk.iter().collect::<String>())
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