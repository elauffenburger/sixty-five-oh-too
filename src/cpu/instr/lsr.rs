use super::super::addr;
use super::super::addr::AddrResult;
use super::InstrResult;
use cpu::Cpu;

use std::fmt;

pub fn acc(cpu: &mut Cpu) -> Box<InstrResult> {
    lsr(addr::acc(cpu), 1, 2)
}

pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::zero_page(cpu);

    lsr(res, 2, 5)
}

pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::zero_page_x(cpu);

    lsr(res, 2, 6)
}

pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::abs(cpu);

    lsr(res, 3, 6)
}

pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::abs_x(cpu);

    lsr(res, 3, 7)
}

pub fn lsr(addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
    Box::new(LsrResult {
        bytes: bytes,
        cycles: cycles,
        addr_result: addr_result
    })
}

struct LsrResult {
    bytes: u8,
    cycles: u8,
    addr_result: AddrResult
}

impl InstrResult for LsrResult {
    fn run(&self, cpu: &mut Cpu) {
        let original_value = self.addr_result.resolve(cpu);

        let new_value = original_value >> 0x01;

        cpu.reg_status.zero = new_value == 0;
        cpu.reg_status.carry = (original_value & 0b0000_0001) == 1;
        cpu.reg_status.negative = ((new_value & 0b1000_0000) >> 7) == 1;

        match &self.addr_result.addr_mode {
            &addr::AddrMode::Accumulator => cpu.reg_acc = new_value as i8,
            _ => cpu.memory.write_at(&self.addr_result.value, &[new_value]),
        }
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for LsrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::debug_fmt("lsr", &self.addr_result))
    }
}