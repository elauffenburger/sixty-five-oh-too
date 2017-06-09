pub mod cmp;
pub mod cpx;
pub mod cpy;

use super::InstrResult;
use super::super::addr;
use self::addr::AddrResult;
use super::Cpu;
use std::fmt;

fn compare(instr_name: &'static str, addr_result: AddrResult, reg_val: i8, bytes: u8, cycles: u8) -> Box<InstrResult> {
    let total_cycles = match addr_result.crosses_boundary.unwrap_or(false) { 
        true => cycles + 1, 
        false => cycles,
    };

    Box::new(CompareInstrResult {
        bytes: bytes,
        cycles: total_cycles,
        reg_val: reg_val,
        addr_result: addr_result,
        instr_name: instr_name
    })
}

struct CompareInstrResult {
    bytes: u8,
    cycles: u8,
    reg_val: i8,
    addr_result: AddrResult,
    instr_name: &'static str
}

impl InstrResult for CompareInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        let mem_val = self.addr_result.resolve(cpu) as i8;

        let (result, _) = self.reg_val.overflowing_sub(mem_val);

        cpu.reg_status.carry = self.reg_val >= mem_val;
        cpu.reg_status.zero = self.reg_val == mem_val;
        cpu.reg_status.negative = result < 0;
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for CompareInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::debug_fmt(self.instr_name, &self.addr_result))
    }
}