use crate::chunk_type::ChunkType;

#[derive(Debug, Clone)]
pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        Self { chunk_type, data }
    }

    fn length(&self) -> usize {
        self.data.len()
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn crc(&self) -> u32 {
        let to_checksum: Vec<_> = self
            .chunk_type
            .bytes()
            .into_iter()
            .chain(self.data().to_owned())
            .collect();
        crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(to_checksum.as_slice())
    }

    pub fn data_as_string(&self) -> crate::error::Result<String> {
        String::from_utf8(self.data.clone()).map_err(|e| e.into())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut byte_str = self.length().to_be_bytes().to_vec();
        byte_str.extend_from_slice(self.data());
        byte_str.extend_from_slice(self.crc().to_be_bytes().as_slice());
        byte_str
    }
}

#[derive(Debug)]
pub struct ChunkParseError {
    message: String,
}

impl std::error::Error for ChunkParseError {}

impl std::fmt::Display for ChunkParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Box<ChunkParseError>;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if let Some((raw_type, raw_data)) = value.split_at_checked(4) {
            match TryInto::<[u8; 4]>::try_into(raw_type) {
                Ok(raw_type) => {
                    let chunk_type = ChunkType::try_from(raw_type).map_err(|e| {
                        Box::new(ChunkParseError {
                            message: format!("invalid chunk type: {e}"),
                        })
                    })?;
                    let chunk = Chunk::new(chunk_type, raw_data[..raw_data.len() - 4].to_vec());
                    let checksum = u32::from_be_bytes(
                        raw_data[raw_data.len() - 4..].try_into().map_err(|e| {
                            Box::new(ChunkParseError {
                                message: format!("invalid checksum length: {e}"),
                            })
                        })?,
                    );
                    if chunk.crc() != checksum {
                        Err(Box::new(ChunkParseError {
                            message: "checksum mismatch!".into(),
                        }))
                    } else {
                        Ok(chunk)
                    }
                }
                Err(e) => Err(Box::new(ChunkParseError {
                    message: format!("{value:?} is an invalid chunk: {e}"),
                })),
            }
        } else {
            Err(Box::new(ChunkParseError {
                message: format!("{value:?} is an invalid chunk: too short"),
            }))
        }
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {:?}", self.chunk_type, self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
