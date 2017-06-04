use super::InstrResult;
use super::Cpu;
use cpu;
use super::addr;

use std::fmt;

extern crate byteorder;
use self::byteorder::{ ByteOrder };

enum ReturnFrom {
    Interrupt,
    Subroutine
}

#[allow(unused_variables)]
pub fn rti(cpu: &mut Cpu) -> Box<InstrResult> {
    ret("rti", ReturnFrom::Interrupt)
}

#[allow(unused_variables)]
pub fn rts(cpu: &mut Cpu) -> Box<InstrResult> {
    ret("rts", ReturnFrom::Subroutine)
}

#[allow(unused_variables)]
fn ret(instr_name: &'static str, from: ReturnFrom) -> Box<InstrResult> {
    Box::new(ReturnInstrResult {
        from: from,
        instr_name: instr_name
    })
}

struct ReturnInstrResult {
    from: ReturnFrom,
    instr_name: &'static str
}

impl InstrResult for ReturnInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        match self.from {
            ReturnFrom::Interrupt => {
                let status = cpu
                    ::ProcessorStatusRegister
                    ::from(cpu.pop_u8().unwrap());
                
                cpu.reg_status = status;
            },
            ReturnFrom::Subroutine => {
                let addr_lo = cpu.pop_u8().unwrap();
                let addr_hi = cpu.pop_u8().unwrap();

                cpu.reg_pc = byteorder::LittleEndian::read_u16(&[addr_lo, addr_hi]);
            }
        }
    } 

    fn get_num_cycles(&self) -> u8 {
       6 
    }
}

impl fmt::Debug for ReturnInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::debug_fmt(self.instr_name, &addr::implicit()))
    }
}