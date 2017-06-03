extern crate byteorder;

use self::byteorder::{ LittleEndian, ByteOrder };

pub fn to_u16(bytes: &[u8]) -> u16 {
    LittleEndian::read_u16(bytes)
}

pub fn test_bit_set(mask: u8, bit: u8) -> bool {
    (mask & 2u8.pow(bit as u32) as u8) >> bit == 1
}

pub fn set_bit(val: u8, bit: u8) -> u8 {
    val | 2u8.pow(bit as u32)
}