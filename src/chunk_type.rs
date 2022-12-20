#[derive(PartialEq, Eq, Debug)]
struct ChunkType([u8; 4]);

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.0
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn is_critical(&self) -> bool {
        is_upper(self.0[0])
    }

    fn is_public(&self) -> bool {
        is_upper(self.0[1])
    }

    fn is_reserved_bit_valid(&self) -> bool {
        is_upper(self.0[2])
    }

    fn is_safe_to_copy(&self) -> bool {
        !is_upper(self.0[3])
    }
}

fn is_upper(byte: u8) -> bool {
    byte & 0b00001000 == 0
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ();

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

impl std::str::FromStr for ChunkType {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let bytes = string
            .bytes()
            .collect::<Vec<u8>>()
            .try_into();
        match bytes {
            Ok(b) => Ok(Self(b)),
            Err(_) => Err(())
        }
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", String::from_utf8(self.0.to_vec()).unwrap());
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    fn test_is_upper() {
        let tests = [
            (0b0000_0000 as u8, false),
            (0b0000_1000 as u8, true)
        ];
        for (byte, exp) in tests.iter() {
            assert_eq!(*exp, is_upper(*byte));
        }
    }

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

