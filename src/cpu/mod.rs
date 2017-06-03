pub mod mem;
pub mod addr;
pub mod instr;

extern crate byteorder;

const NMI_VECTOR_ADDR: &'static [u16] = &[0xfffa, 0xfffb];
const RESET_VECTOR_ADDR: &'static[u16] = &[0xfffc, 0xfffd];
const IRQ_BRK_VECTOR_ADDR: &'static[u16] = &[0xffe, 0xffff];
const STACK_POINTER_START_ADDR: u16 = 0x0100;

pub enum Register {
    A,
    X,
    Y
}

pub struct Cpu {
    pub reg_acc: i8,
    pub reg_x: i8,
    pub reg_y: i8,
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
            reg_sp: 0xff,

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
        let indirect_pc_addr = self.memory.read_u16_at(&RESET_VECTOR_ADDR[0]);
        let direct_pc_addr = self.memory.read_u16_at(&indirect_pc_addr);

        self.reg_pc = direct_pc_addr;
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

        self.memory.read_u8_at(&addr)
    }

    fn get_real_sp_addr(&self) -> u16 {
        STACK_POINTER_START_ADDR + (self.reg_sp as u16)
    }

    pub fn push_u8(&mut self, val: u8) {
        let addr = self.get_real_sp_addr();
        self.memory.mem[addr as usize] = val;

        self.reg_sp -= 1;
    }

    pub fn pop_u8(&mut self) -> Option<u8> {
        self.reg_sp += 1;

        let addr = self.get_real_sp_addr();
        if addr > 0x01ff {
            return None;
        }

        let val = self.memory.mem[addr as usize];
        Some(val)
    }
}

#[derive(Clone)]
pub struct ProcessorStatusRegister {
    negative: bool,
    overflow: bool,

    brk: bool,
    decimal_mode: bool,
    irq_disable: bool,
    zero: bool,
    carry: bool
}

impl Into<u8> for ProcessorStatusRegister {
    fn into(self) -> u8 {
        let mut result = 0b0010_0000;

        if self.negative {
            result |= 0b1000_0000;
        }

        if self.overflow {
            result |= 0b0100_0000;
        }

        if self.brk {
            result |= 0b0001_0000;
        }

        if self.decimal_mode {
            result |= 0b0000_1000;
        }

        if self.irq_disable {
            result |= 0b0000_0100;
        }

        if self.zero {
            result |= 0b0000_0010;
        }

        if self.carry {
            result |= 0b0000_0001;
        }

        result
    }
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
    use super::super::util;

    #[test]
    pub fn endianness() {
        let mut rdr = Cursor::new(vec![0x00, 0x10]);

        assert_eq!(0x1000, rdr.read_u16::<LittleEndian>().unwrap());
    }

    #[test]
    pub fn bit_set() {
        assert!(util::test_bit_set(0b1000_0000, 7));
        assert!(!util::test_bit_set(0b1000_0000, 6));
        assert!(util::test_bit_set(0b1111_1111, 2));
        assert!(!util::test_bit_set(0b1110_1111, 4));
    }

    #[test]
    pub fn push_u8() {
        let mut cpu = Cpu::new();

        cpu.push_u8(0x01);
        cpu.push_u8(0x02);
        cpu.push_u8(0x03);

        assert_eq!(cpu.reg_sp, 0xfc);

        assert_eq!(cpu.pop_u8(), Some(0x03));
        assert_eq!(cpu.reg_sp, 0xfd);

        assert_eq!(cpu.pop_u8(), Some(0x02));
        assert_eq!(cpu.reg_sp, 0xfe);

        assert_eq!(cpu.pop_u8(), Some(0x01));
        assert_eq!(cpu.reg_sp, 0xff);
    }
}