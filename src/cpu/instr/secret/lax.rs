use super::addr;
use super::instr;
use super::cpu;

use std::fmt;

pub fn zero_page(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    lax(addr::zero_page(cpu), 2, 3)
}

pub fn zero_page_y(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    lax(addr::zero_page_y(cpu), 2, 4)
}

pub fn abs(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    lax(addr::abs(cpu), 3, 4)
}

pub fn abs_y(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    lax(addr::abs_y(cpu), 3, 4)
}

pub fn ind_x(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    lax(addr::ind_x(cpu), 2, 6)
}

pub fn ind_y(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    lax(addr::ind_y(cpu), 2, 5)
}

fn lax(addr_result: addr::AddrResult, bytes: u8, cycles: u8) -> Box<instr::InstrResult> {
    let final_cycles = match addr_result.crosses_boundary {
        Some(true) => cycles + 1,
        _ => cycles
    };

    Box::new(LaxInstrResult {
        bytes: bytes,
        cycles: final_cycles,
        addr_result: addr_result
    })
}

struct LaxInstrResult {
    bytes: u8,
    cycles: u8,
    addr_result: addr::AddrResult
}

impl instr::InstrResult for LaxInstrResult {
    fn run(&self, cpu: &mut cpu::Cpu) {
        let result = self.addr_result.resolve(cpu) as i8;

        cpu.reg_acc = result;
        cpu.reg_x = result;

        cpu.reg_status.negative = result < 0;
        cpu.reg_status.zero = result == 0;
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for LaxInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::instr::debug_fmt("lax", &self.addr_result))
    }
}