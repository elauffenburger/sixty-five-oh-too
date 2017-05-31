pub mod mem;
pub mod addr;
pub mod instr;

extern crate byteorder;

use std::io::Cursor;
use self::byteorder::{ LittleEndian, ReadBytesExt, ByteOrder };

const NMI_VECTOR_ADDR: &'static [u8] = &[0xfffa, 0xfffb];
const RESET_VECTOR_ADDR: &'static[u8] = &[0xfffc, 0xfffd];
const IRQ_BRK_VECTOR_ADDR: &'static[u8] = &[0xffe, 0xffff];

pub struct Cpu {
    pub reg_acc: u8,
    pub reg_x: u8,
    pub reg_y: u8,
    pub reg_pc: u16,
    pub reg_sp: u8,

    pub reg_status: ProcessorStatusRegister,
    pub memory: mem::MemoryMap
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            reg_acc: 0,
            reg_x: 0,
            reg_y: 0,
            reg_pc: 0,
            reg_sp: 0,

            reg_status: ProcessorStatusRegister::default(),
            memory: mem::MemoryMap::default()
        }
    }
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu::default();
        cpu.reset();

        cpu
    }

    pub fn reset(&mut self) {
        let indirect_pc_addr = Cpu::to_u16(RESET_VECTOR_ADDR);
        let direct_pc_addr = self.memory.read_u16_at(&indirect_pc_addr);

        self.reg_pc = direct_pc_addr;
    }

    pub fn to_u16(bytes: &[u8]) -> u16 {
        LittleEndian::read_u16(bytes)
    }

    pub fn read_u8(&mut self) -> u8 {
        let val = self.memory.read_u8_at(&self.reg_pc);
        self.reg_pc += 0x1;

        val
    }

    pub fn read_u16(&mut self) -> u16 {
        let val = self.memory.read_u16_at(&self.reg_pc);
        self.reg_pc += 0x1;

        val
    }

    pub fn deref_u8(&mut self) -> u8 {
        let addr = self.read_u8() as u16;

        self.read_u8()
    }

    pub fn test_bit_set(mask: u8, bit: u8) -> bool {
        (mask & 2u8.pow(bit as u32) as u8) >> bit == 1
    }
}

pub struct ProcessorStatusRegister {
    negative: bool,
    overflow: bool,

    brk: bool,
    decimal_mode: bool,
    irq_disable: bool,
    zero: bool,
    carry: bool
}

impl Default for ProcessorStatusRegister {
    fn default() -> Self {
        ProcessorStatusRegister {
            negative: false,
            overflow: false,
            brk: false,
            decimal_mode: false,
            irq_disable: false,
            zero: false,
            carry: false
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::byteorder::{LittleEndian, ReadBytesExt};
    use super::Cpu;

    #[test]
    pub fn endianness() {
        let mut rdr = Cursor::new(vec![0x00, 0x10]);

        assert_eq!(0x1000, rdr.read_u16::<LittleEndian>().unwrap());
    }

    #[test]
    pub fn bit_set() {
        assert!(Cpu::test_bit_set(0b1000_0000, 7));
        assert!(!Cpu::test_bit_set(0b1000_0000, 6));
        assert!(Cpu::test_bit_set(0b1111_1111, 2));
        assert!(!Cpu::test_bit_set(0b1110_1111, 4));
    }
}