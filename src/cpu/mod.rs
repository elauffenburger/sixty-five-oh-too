extern crate byteorder;

pub mod mem;
pub mod addr;
pub mod instr;

mod status_reg;

pub use self::status_reg::ProcessorStatusRegister;

use util;
use self::instr::resolver;

const NMI_VECTOR_ADDR: &'static [u16] = &[0xfffa, 0xfffb];
const RESET_VECTOR_ADDR: &'static [u16] = &[0xfffc, 0xfffd];
pub const IRQ_BRK_VECTOR_ADDR: &'static [u16] = &[0xffe, 0xffff];
const STACK_POINTER_START_ADDR: u16 = 0x0100;

pub enum Register {
    A,
    X,
    Y,
    SP,
}

pub struct Cpu {
    pub reg_acc: i8,
    pub reg_x: i8,
    pub reg_y: i8,
    pub reg_pc: u16,
    pub reg_sp: u8,

    pub reg_status: ProcessorStatusRegister,
    pub memory: mem::MemoryMap,

    pub pending_cycles: Option<u8>,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            reg_acc: 0,
            reg_x: 0,
            reg_y: 0,
            reg_pc: 0,
            reg_sp: 0xfd,

            reg_status: ProcessorStatusRegister::default(),
            memory: mem::MemoryMap::default(),

            pending_cycles: None,
        }
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu::default()
    }

    pub fn load_program(&mut self, start_addr: u16, bytes: &[u8]) {
        let start_hi = ((start_addr & 0xff00) >> 8) as u8;
        let start_lo = (start_addr & 0x00ff) as u8;

        // write program to memory
        self.memory.write_at(&start_addr, bytes);

        // update reset vector to point to starting addr
        self.memory.write_at(&RESET_VECTOR_ADDR[0], &[start_lo, start_hi]);

        // update brk vector to point to end of rom by default (so programs will exit on brk by default)
        self.memory.write_at(&IRQ_BRK_VECTOR_ADDR[0], &[0xff, 0xff]);
    }

    pub fn reset(&mut self) {
        let address = self.memory.read_u16_at(&RESET_VECTOR_ADDR[0]);

        self.reg_pc = address;
    }

    pub fn read_u8(&mut self) -> u8 {
        let val = self.memory.read_u8_at(&self.reg_pc);
        self.reg_pc += 0x1;

        val
    }

    pub fn read_u16(&mut self) -> u16 {
        let val = self.memory.read_u16_at(&self.reg_pc);
        self.reg_pc += 0x2;

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

    pub fn run(&mut self) {
        println!("starting execution...");

        self.reset();

        'main: loop {
            match self.step() {
                true => {}
                false => break 'main,
            }
        }

        println!("finished execution!");
    }

    pub fn step(&mut self) -> bool {
        let start_pc = self.reg_pc;

        let should_delay = match self.pending_cycles {
            None => false,
            Some(cycles) => {
                self.pending_cycles = match cycles - 1 {
                    0 => None,
                    cycles => Some(cycles),
                };

                true
            }
        };

        if should_delay {
            return true;
        }

        match self.next_instr() {
            None => false,
            Some(opcode) => {
                match resolver::resolve(opcode) {
                    None => panic!("failed to resolve opcode {:x}!", opcode),
                    Some(instr) => {
                        let instr_result = (instr)(self);

                        let instr_str = format!("{0:<4x}\t{1:<2x}\t{2:?}", start_pc, opcode, instr_result);
                        println!("{0:<30}\tA:{1:02x}, X:{2:02x}, Y:{3:02x}, P:{4:02?}, SP:{5:02x}",
                                 instr_str,
                                 self.reg_acc,
                                 self.reg_x,
                                 self.reg_y,
                                 self.reg_status,
                                 self.reg_sp);

                        (*instr_result).run(self);

                        let cycles = self.pending_cycles.unwrap_or(0) + instr_result.get_num_cycles();
                        self.pending_cycles = Some(cycles);

                        true
                    }
                }
            }
        }
    }

    fn next_instr(&mut self) -> Option<u8> {
        match self.reg_pc < 0xffff {
            true => Some(self.read_u8()),
            false => None,
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

    #[test]
    pub fn load_program() {
        let mut cpu = Cpu::new();

        // lda #$11
        cpu.load_program(0x6000, &[0xa9, 0x11]);

        assert_eq!(cpu.memory.read_u16_at(&super::RESET_VECTOR_ADDR[0]), 0x6000);
    }
}
