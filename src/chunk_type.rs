#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(dead_code)]

use std::fmt::Display;
use std::io::Read;
use std::str::FromStr;
use std::num::ParseIntError;
use std::cmp::PartialEq;


pub fn read_header(filename: &String) -> Vec<u8> {
    let mut header = vec![0u8; 8];
    let mut file = std::fs::File::open(&filename)
    .expect("No file Found");

    file.read_exact(&mut header);
    print_header(&header);
    return header;
}

pub fn print_header(tab: &Vec<u8>) {
    println!("Header {:?}", &tab);
}

pub fn check_if_vaid_png(filename: &String) -> bool {
    let pngHeader: Vec<u8> = read_header(&filename); 
    let validHeader: Vec<u8> = vec![137, 80, 78, 71, 13, 10, 26, 10]; 
    return pngHeader == validHeader;
}
#[derive(Eq, Debug)]
pub struct ChunkType {
    pub chunk: [u8;4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        return self.chunk;
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..4 {
            if !((65 < self.chunk[i] && self.chunk[i] < 90) || (97 < self.chunk[i] && self.chunk[i] < 122)) {
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
            if !((65 < value[i] && value[i] < 90) 
            ||(97 < value[i] && value[i] < 122)) {
                return Err("Value not ACSII");
            }
        }
        return Ok(ChunkType { chunk: value });
    }
}

impl FromStr for ChunkType {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
/*
impl Eq for ChunkType {
    fn assert_receiver_is_total_eq(&self) {
        assert_eq!()
    }
}
*/
impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = std::str::from_utf8(&self.chunk).unwrap();
        return write!(f, "{}", text);
    }
}

