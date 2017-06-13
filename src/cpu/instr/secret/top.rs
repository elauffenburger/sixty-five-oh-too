use super::{Cpu, InstrResult, AddrResult};
use super::addr;

use std::fmt;

#[allow(unused_variables)]
pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
    top(addr::abs(cpu), 3, 4)
}

#[allow(unused_variables)]
pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
    top(addr::abs_x(cpu), 3, 4)
}

fn top(addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
    let final_cycles = match addr_result.crosses_boundary.unwrap_or(false) {
        true => cycles + 1,
        false => cycles,
    };

    Box::new(TopInstrResult {
        bytes: bytes,
        cycles: final_cycles,
        addr_result: addr_result,
    })
}

struct TopInstrResult {
    bytes: u8,
    cycles: u8,
    addr_result: AddrResult,
}

impl InstrResult for TopInstrResult {
    #[allow(unused_variables)]
    fn run(&self, cpu: &mut Cpu) {}

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for TopInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::super::debug_fmt("top", &self.addr_result))
    }
}