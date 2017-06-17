use super::addr;
use super::instr;
use super::cpu;

use super::instr::dec;
use super::instr::compare::cmp;

use std::fmt;

pub fn zero_page(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    rla(addr::zero_page(cpu), 2, 5)
}

pub fn zero_page_x(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    rla(addr::zero_page_x(cpu), 2, 6)
}

pub fn abs(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    rla(addr::abs(cpu), 3, 6)
}

pub fn abs_x(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    rla(addr::abs_x(cpu), 3, 7)
}

pub fn abs_y(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    rla(addr::abs_y(cpu), 3, 7)
}

pub fn ind_x(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    rla(addr::ind_x(cpu), 2, 8)
}

pub fn ind_y(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    rla(addr::ind_y(cpu), 2, 8)
}

fn rla(addr_result: addr::AddrResult, bytes: u8, cycles: u8) -> Box<instr::InstrResult> {
    Box::new(RlaInstrResult {
        bytes: bytes,
        cycles: cycles,
        addr_result: addr_result,
    })
}

struct RlaInstrResult {
    bytes: u8,
    cycles: u8,
    addr_result: addr::AddrResult,
}

impl instr::InstrResult for RlaInstrResult {
    fn run(&self, cpu: &mut cpu::Cpu) {
        let rol = super::instr::rotate::rol::rol(self.addr_result.clone(), 0, 0);
        let and = super::instr::and::and(cpu, self.addr_result.clone(), 0, 0);

        (*rol).run(cpu);
        (*and).run(cpu);
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for RlaInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::instr::debug_fmt("rla", &self.addr_result))
    }
}