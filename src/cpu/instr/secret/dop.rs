use super::{Cpu, InstrResult, AddrResult};
use super::addr;

use std::fmt;

#[allow(unused_variables)]
pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
    dop(addr::zero_page(cpu), 2, 3)
}

#[allow(unused_variables)]
pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
    dop(addr::zero_page_x(cpu), 2, 4)
}

#[allow(unused_variables)]
pub fn imm(cpu: &mut Cpu) -> Box<InstrResult> {
    dop(addr::imm(cpu), 2, 2)
}

fn dop(addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
    Box::new(DopInstrResult {
        bytes: bytes,
        cycles: cycles,
        addr_result: addr_result,
    })
}

struct DopInstrResult {
    bytes: u8,
    cycles: u8,
    addr_result: AddrResult,
}

impl InstrResult for DopInstrResult {
    #[allow(unused_variables)]
    fn run(&self, cpu: &mut Cpu) {}

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for DopInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::super::debug_fmt("dop", &self.addr_result))
    }
}