use std::fmt::Display;
use std::str::FromStr;
use std::num::ParseIntError;
use std::cmp::PartialEq;

#[derive(Eq, Debug)]
pub struct ChunkType {
    pub chunk: [u8;4],
}

impl ChunkType {
    pub const CHUNK_TYPE_LENGHT:usize = 4;

    pub fn bytes(&self) -> [u8; 4] {
        return self.chunk;
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..4 {
            if !((65 <= self.chunk[i] && self.chunk[i] <= 90) || (97 <= self.chunk[i] && self.chunk[i] <= 122)) {
                return false;
            }
        } 
        return true;
    }

    fn get_bit(&self, byteIndex: usize, bitNumber: u8) -> u8 {
        let bit : u8 = self.chunk[byteIndex] as u8;
        let xbit :u8 = (bit >> bitNumber) & 1; 
        return xbit;
    }
/*
Ancillary bit: bit 5 of first byte
0 (uppercase) = critical, 1 (lowercase) = ancillary.
*/
    pub fn is_critical(&self) -> bool {
        let xbit = self.get_bit(0, 5);
        if (xbit == 0) {
            return true;
        }
        return false;
    }
/* 
Private bit: bit 5 of second byte
0 (uppercase) = public, 1 (lowercase) = private.
    */
    pub fn is_public(&self) -> bool {
        let xbit = self.get_bit(1, 5);
        if (xbit == 0) {
            return true;
        }
        return false;
    }
/*
Reserved bit: bit 5 of third byte
Must be 0 (uppercase) in files conforming to this version of PNG.
 */
    pub fn is_reserved_bit_valid(&self) -> bool {
        let xbit = self.get_bit(2, 5);
        if (xbit == 0) {
            return true;
        }
        return false;
    }
/*
Safe-to-copy bit: bit 5 of fourth byte
0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy.
*/
    pub fn is_safe_to_copy(&self) -> bool {
        let xbit = self.get_bit(3, 5);
        if (xbit == 1) {
            return true;
        }
        return false;
    }

}

impl TryFrom<[u8;4]> for ChunkType {
    type Error = &'static str;
    
    fn try_from(value: [u8;4]) -> Result<Self, Self::Error> {
        for i in 0..4 {
            if !((65 <= value[i] && value[i] <= 90) 
            ||(97 <= value[i] && value[i] <= 122)) {
                println!("Error Value = {}", value[i]);
                return Err("Error::ChunkType::Value not ACSII");
            }
        }
        return Ok(ChunkType { chunk: value });
    }
}

impl FromStr for ChunkType {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(s.len() == ChunkType::CHUNK_TYPE_LENGHT);
        let mut value: [u8;4] = [0,0,0,0];
        s.bytes().zip(value.iter_mut()).for_each(|(b,ptr)| *ptr = b);
        return Ok(ChunkType {chunk: value});
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.chunk.len() {
            if (self.chunk[i] != other.chunk[i]) {
                return false;
            }
        }
        return true;
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = std::str::from_utf8(&self.chunk).unwrap();
        return write!(f, "{}", text);
    }
}

