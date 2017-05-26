mod mem;
mod addr;
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

    #[test]
    pub fn test_endianness() {
        let mut rdr = Cursor::new(vec![0x00, 0x10]);

        assert_eq!(0x1000, rdr.read_u16::<LittleEndian>().unwrap());
    }
}