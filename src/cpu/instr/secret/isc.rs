use super::addr;
use super::instr;
use super::cpu;

use super::instr::inc;
use super::instr::sbc;

use std::fmt;

pub fn zero_page(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    isc(addr::zero_page(cpu), 2, 5)
}

pub fn zero_page_x(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    isc(addr::zero_page_x(cpu), 2, 6)
}

pub fn abs(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    isc(addr::abs(cpu), 3, 6)
}

pub fn abs_x(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    isc(addr::abs_x(cpu), 3, 7)
}

pub fn abs_y(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    isc(addr::abs_y(cpu), 3, 7)
}

pub fn ind_x(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    isc(addr::ind_x(cpu), 2, 8)
}

pub fn ind_y(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    isc(addr::ind_y(cpu), 2, 8)
}

fn isc(addr_result: addr::AddrResult, bytes: u8, cycles: u8) -> Box<instr::InstrResult> {
    Box::new(IscInstrResult {
        bytes: bytes,
        cycles: cycles,
        addr_result: addr_result
    })
}

struct IscInstrResult {
    bytes: u8,
    cycles: u8,
    addr_result: addr::AddrResult
}

impl instr::InstrResult for IscInstrResult {
    fn run(&self, cpu: &mut cpu::Cpu) {
        let inc = inc::inc(inc::IncrementType::Memory(self.addr_result.clone()), 0, 0);
        let sbc = sbc::sbc(self.addr_result.clone(), 0, 0);

        (*inc).run(cpu);
        (*sbc).run(cpu);
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for IscInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::instr::debug_fmt("isc", &self.addr_result))
    }
}