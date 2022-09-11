use std::convert::TryFrom;
use crate::chunk::Chunk;

pub struct Png {
    header: [u8;8],
    bytes: Vec<Chunk>,
}

impl Png {
    pub const STANDARD_HEADER:[u8;8] = [137,80,78,71,13,10,26,10];

    pub fn from_chunks(chunk: Vec<Chunk>) -> Png {
        return Png{ header: Png::STANDARD_HEADER ,bytes: chunk }
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.bytes.push(chunk);
    }

    /*
    pub fn remove_chunk(&mut self, chunkT: &str) -> Result<Chunk> {
        let index = xs.iter().position(|x| *x == some_x).unwrap();
        xs.remove(index);
    }
    */

    pub fn header(&self) -> &[u8;8] {
        return &self.header;
    }

    /*
    pub fn chunks(&self) -> &[Chunk] {
        return &self.bytes.as_slice();    
    }
    */
    
}


impl TryFrom<&[u8]> for Png {
    type Error = &'static str;
    fn try_from(_value: &[u8]) -> Result<Self, Self::Error> {   
        let v = Vec::new();
        return Ok(Png { header: Png::STANDARD_HEADER ,bytes: v });
    }
}