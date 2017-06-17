use super::addr;
use super::instr;
use super::cpu;

use std::fmt;

pub fn zero_page(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    aax(addr::zero_page(cpu), 2, 3)
}

pub fn zero_page_y(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    aax(addr::zero_page_y(cpu), 2, 4)
}

pub fn abs(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    aax(addr::abs(cpu), 3, 4)
}

pub fn ind_x(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    aax(addr::ind_x(cpu), 2, 6)
}

fn aax(addr_result: addr::AddrResult, bytes: u8, cycles: u8) -> Box<instr::InstrResult> {
    let final_cycles = match addr_result.crosses_boundary {
        Some(true) => cycles + 1,
        _ => cycles
    };

    Box::new(AaxInstrResult {
        bytes: bytes,
        cycles: final_cycles,
        addr_result: addr_result
    })
}

struct AaxInstrResult {
    bytes: u8,
    cycles: u8,
    addr_result: addr::AddrResult
}

impl instr::InstrResult for AaxInstrResult {
    fn run(&self, cpu: &mut cpu::Cpu) {
        let result = cpu.reg_acc & cpu.reg_x;

        cpu.reg_status.negative = result < 0;
        cpu.reg_status.zero = result == 0;

        cpu.memory.write_at(&self.addr_result.value, &[result as u8]);
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for AaxInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::instr::debug_fmt("aax", &self.addr_result))
    }
}