extern crate byteorder;

use self::byteorder::{LittleEndian, ByteOrder};

pub fn to_u16(bytes: &[u8]) -> u16 {
    LittleEndian::read_u16(bytes)
}

pub fn test_bit_set(mask: u8, bit: u8) -> bool {
    (mask & 2u8.pow(bit as u32) as u8) >> bit == 1
}

pub fn set_bit(val: u8, bit: u8, set: bool) -> u8 {
    let mask = 2u8.pow(bit as u32);

    match set {
        true => val | mask,
        false => val & (!mask),
    }
}

#[cfg(test)]
pub mod test {
    use super::set_bit;

    #[test]
    fn test_set_bit() {
        assert_eq!(0b0010_0101, set_bit(0b0000_0101, 5, true));
        assert_eq!(0b0010_0100, set_bit(0b0010_0101, 0, false));
    }
}
