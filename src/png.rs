use std::convert::TryFrom;
use std::fmt::Display;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Png {
    header: [u8;8],
    bytes: Vec<Chunk>,
}

impl Png {
    pub const STANDARD_HEADER:[u8;8] = [137, 80, 78, 71, 13, 10, 26, 10];
    //type Error = &'static str;
    pub fn from_chunks(chunk: Vec<Chunk>) -> Png {
        return Png{ header: Png::STANDARD_HEADER ,bytes: chunk }
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.bytes.push(chunk);
    }

    
    pub fn remove_chunk(&mut self, chunkT: &str) -> Result<Chunk, &'static str> {
        let index = self.bytes.iter().position(|x| *x.chunk_type() == ChunkType::from_str(chunkT).unwrap()).unwrap();
        return Ok(self.bytes.remove(index));
    }
    

    pub fn header(&self) -> &[u8;8] {
        return &self.header;
    }

    pub fn chunks(&self) -> &[Chunk] {
        return &self.bytes.as_slice();    
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        let index = self.bytes.iter()
        .position(|x| *x.chunk_type() == ChunkType::from_str(chunk_type).unwrap());
        if (index == None) {
            return None
        }
        return Some(&self.bytes[index.unwrap()]);
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut tab:Vec<u8> = Vec::new();
        for value in Png::STANDARD_HEADER {
            tab.push(value);
        }
        for chunk in &self.bytes {
            println!("Chunks: {}", chunk);
            for i in chunk.as_bytes() {
                tab.push(i);
            }
            
        }
        return tab;
    }

    pub fn print(&self) {
        println!("Header: {:?}", self.header);
        for c in &self.bytes {
            println!("{}", c);
        }       
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut v = Vec::new();
        for i in 0..Png::STANDARD_HEADER.len() {
            print!("{} ", value[i]);
            if (value[i] != Png::STANDARD_HEADER[i]) {
                return Err("Invalid png header");
            }
        }
        print!("\n");
        assert!(value.len() >= Png::STANDARD_HEADER.len());

        let mut i: usize = Png::STANDARD_HEADER.len();
        while i < value.len() {
            //println!("lenght = {} {} {} {}", value[i],value[i+1],value[i+2],value[i+3]);
            let length :u32 = u32::from_be_bytes([value[i],value[i+1],value[i+2],value[i+3]]);
            let c = ChunkType::try_from([value[i+4], value[i+5], value[i+6], value[i+7]]).unwrap();
            i = i + Chunk::DATA_LENGTH_BYTES + Chunk::CHUNK_TYPE_BYTES;
            
            let mut data: Vec<u8> = Vec::new();
            for j in i..(length as usize + i) {
                data.push(value[j]);
            }
            let ch = Chunk::new(c, data);
            v.push(ch);
            i = i + length as usize + Chunk::CRC_BYTES;
        }
        /*
        for mut i in Png::STANDARD_HEADER.len()..value.len() {
            println!("lenght = {} {} {} {}", value[i],value[i+1],value[i+2],value[i+3]);
            let length :u32 = u32::from_be_bytes([value[i],value[i+1],value[i+2],value[i+3]]);
            println!("lenght = {}", length);
            let c = ChunkType::try_from([value[i+4], value[i+5], value[i+6], value[i+7]]).unwrap();
            i = i + Chunk::DATA_LENGTH_BYTES + Chunk::CHUNK_TYPE_BYTES;
            let mut data: Vec<u8> = Vec::new();
            for j in i..(length as usize + i - 1) {
                /*
                data.push(value[j]);
                */
            }
            let ch = Chunk::new(c, data);
            v.push(ch);
            i = i + length as usize + Chunk::CRC_BYTES;
        }
        */
        return Ok(Png { header: Png::STANDARD_HEADER ,bytes: v });
    }
}


impl Display for Png {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Header: {:?}\nData: {:?}\n", self.header, self.bytes);
    }
}
