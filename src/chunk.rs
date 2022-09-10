use std::{
    fmt::Display, 
    convert::{
        TryFrom, 
        TryInto
    }
};

use crate::Error;

use super::chunk_type::ChunkType;
use crc::{Crc, CRC_32_ISO_HDLC};

#[derive(Debug, PartialEq)]
pub struct Chunk {
    chunkT: ChunkType, //4bytes bb bb bb bb
    data: Vec<u8>, //any bytes
}

impl Chunk {
    pub const DATA_LENGTH_BYTES: usize = 4;
    pub const CHUNK_TYPE_BYTES: usize = 4;
    pub const CRC_BYTES: usize = 4;
    
    pub const METADATA_LENGHT:usize = Chunk::DATA_LENGTH_BYTES
        + Chunk::CHUNK_TYPE_BYTES
        + Chunk::CRC_BYTES;

    pub fn new(chunk: ChunkType, data: Vec<u8>) -> Chunk {
        return Chunk { chunkT: chunk, data: data};
    }
    
    pub fn length(&self) -> u32 {
        return self.data.len() as u32;
    }
    
    pub fn chunk_type(&self) -> &ChunkType {
        return &self.chunkT;
    }

    pub fn data(&self) -> &Vec<u8> {
        return &self.data;
    }

    pub fn crc(&self) -> u32 {
        pub const ISO_CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        //let bytes:Vec<u8> = vec![4,5];
        let bytes: Vec<u8> = self
        .chunkT
        .bytes()
        .iter()
        .chain(self.data.iter())
        .copied()
        .collect();
        return ISO_CRC.checksum(&bytes);
    }

    pub fn data_as_string(&self) -> Result<String, Error> {
        let s:String = 
        std::str::from_utf8(&self.data)
        .unwrap()
        .to_string();

        return Ok(s);
    }
    fn as_bytes(&self) -> Vec<u8> {
        let v:Vec<u8> = Vec::new();
        return v;
    }

}

impl Display for Chunk{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, 
            "Chunk::\nchunk_type: {}\ndata: {:?}\n", self.chunkT, self.data);
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;
    
    //          bb          bb          bb        bb
    //example   12          01          02        12
    //      00001100    00000001    00000010    00001100
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let len = value.len();
        /*
        let length :u32 =
            value[0] as u32
        + ((value[1] as u32) << 8)
        + ((value[2] as u32) << 16)
        + ((value[3] as u32) << 24);
        */
        let chunk = ChunkType::try_from([value[4], value[5], value[6], value[7]]).unwrap();
        let mut data: Vec<u8> = Vec::new();
        for i in 8..len-4 {
            data.push(value[i]);
        }
        let cc: Chunk = Chunk { chunkT: chunk, data: data };
        return Ok(cc);
    }
}