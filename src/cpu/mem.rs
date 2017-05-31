extern crate byteorder;

use self::byteorder::{ LittleEndian, ByteOrder };

pub struct MemoryMap {
    pub mem: Vec<u8>,
}

impl Default for MemoryMap {
    fn default() -> Self {
        MemoryMap {
            mem: vec![0;0xffff]
        }
    }
}

impl MemoryMap {
    pub fn read_u16_at(&self, addr: &u16) -> u16 {
        let buf = &[self.mem[*addr as usize], self.mem[(*addr + 1) as usize]];

        LittleEndian::read_u16(buf)
    }

    pub fn read_u8_at(&self, addr: &u16) -> u8 {
        self.mem[*addr as usize]
    }

    pub fn deref_u8_at(&self, addr: &u16) -> u8 {
        let indirect_addr = self.read_u8_at(addr);

        self.read_u8_at(&(indirect_addr as u16))
    }

    pub fn deref_u16_at(&self, addr: &u16) -> u16 {
        let indirect_addr = self.read_u16_at(addr);

        self.read_u16_at(&indirect_addr)
    }

    pub fn write_at(&mut self, start_addr: &u16, bytes: &[u8]) {
        let mut i = 0;
        for byte in bytes {
            self.mem[(start_addr + i) as usize] = *byte;

            i += 1;
        }
    }

    pub fn crosses_page_boundary(addr_one: &u16, addr_two: &u16) -> bool {
        (addr_one & 0xff00) >> 8 != (addr_two & 0xff00) >> 8
    }
}

#[cfg(test)]
mod test {
    use super::MemoryMap;

    #[test]
    pub fn crosses_page_boundary() {
        assert!(MemoryMap::crosses_page_boundary(&0x01FF, &0x0200));
        assert!(!MemoryMap::crosses_page_boundary(&0x01FF, &0x01FE));
    }
}