use std::{fmt::Display, convert::TryFrom};
use crate::Error;
use super::chunk_type::ChunkType;
use crc::{Crc, CRC_32_ISO_HDLC};

/*

From PNG Specs:
http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html#CRC-algorithm

Each chunk consists of four parts:

--> Length: A 4-byte unsigned integer giving the number of bytes 
in the chunk's data field.The length counts only the data field, 
not itself, the chunk type code, or the CRC. Zero is a valid length.
Although encoders and decoders should treat the length as unsigned, 
its value must not exceed 231 bytes.

--> Chunk Type: A 4-byte chunk type code. For convenience in description
 and in examining PNG files, type codes are restricted to consist of
  uppercase and lowercase ASCII letters (A-Z and a-z, or 65-90 and 
97-122 decimal). 

-->Chunk Data: The data bytes appropriate to the chunk type, 
if any. This field can be of zero length.

--> CRC A 4-byte CRC (Cyclic Redundancy Check) calculated on the 
preceding bytes in the chunk, including the chunk type code and 
chunk data fields, but not including the length field.
The CRC is always present, even for chunks containing no data.

Chunk CRCs are calculated using standard CRC methods with 
pre and post conditioning, as defined by ISO 3309
*/

#[derive(Debug, PartialEq)]
pub struct Chunk {
    chunkT: ChunkType, //4bytes bb bb bb bb
    data: Vec<u8>, //any bytes
}

fn u32_to_u8_array(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
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
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut v:Vec<u8> = Vec::new();
        let len : u32 = self.length();
        v.extend_from_slice(&u32_to_u8_array(len));
        let typeT : &ChunkType = &self.chunkT;
        v.extend_from_slice(&typeT.bytes());
        for i in 0..self.data.len() {
            v.push(self.data[i]);
        }
        let crc : u32 = self.crc();
        v.extend_from_slice(&u32_to_u8_array(crc));
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
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let len = bytes.len();
        
        let chunk = ChunkType::try_from([bytes[4], bytes[5], bytes[6], bytes[7]]).unwrap();
        let mut data: Vec<u8> = Vec::new();
        for i in 8..len-4 {
            data.push(bytes[i]);
        }
        let cc: Chunk = Chunk { chunkT: chunk, data: data };
        
        let length :u32 =
            bytes[3] as u32
        + ((bytes[2] as u32) << 8)
        + ((bytes[1] as u32) << 16)
        + ((bytes[0] as u32) << 24);
        //println!("Size == {}" , length);
        if (len - Chunk::METADATA_LENGHT != length as usize) {
            //return Err("Error::Length bytes mismatch");
        }
        let crc :u32 =
            bytes[len-1] as u32
        + ((bytes[len-2] as u32) << 8)
        + ((bytes[len-3] as u32) << 16)
        + ((bytes[len-4] as u32) << 24);
        /*let crc:u32 = u32::from_be_bytes([
            bytes[len-4], 
            bytes[len-3], 
            bytes[len-2], 
            bytes[len-1]
        ]);
        */
        //println!("Crc == {}" , crc);
        
        if (cc.crc() != crc) {
            return Err("Error::crc bytes mismatch");
        }

        return Ok(cc);
    }
}