use super::addr;
use super::instr;
use super::cpu;

use std::fmt;

pub fn zero_page(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    dcp(addr::zero_page(cpu), 2, 5)
}

pub fn zero_page_x(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    dcp(addr::zero_page_x(cpu), 2, 6)
}

pub fn abs(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    dcp(addr::abs(cpu), 3, 6)
}

pub fn abs_x(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    dcp(addr::abs_x(cpu), 3, 7)
}

pub fn abs_y(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    dcp(addr::abs_y(cpu), 3, 7)
}

pub fn ind_x(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    dcp(addr::ind_x(cpu), 2, 8)
}

pub fn ind_y(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    dcp(addr::ind_y(cpu), 2, 8)
}

fn dcp(addr_result: addr::AddrResult, bytes: u8, cycles: u8) -> Box<instr::InstrResult> {
    Box::new(DcpInstrResult {
        bytes: bytes,
        cycles: cycles,
        addr_result: addr_result
    })
}

struct DcpInstrResult {
    bytes: u8,
    cycles: u8,
    addr_result: addr::AddrResult
}

impl instr::InstrResult for DcpInstrResult {
    fn run(&self, cpu: &mut cpu::Cpu) {
        let value = self.addr_result.resolve(cpu) as i8;
        let (new_value, _) = value.overflowing_sub(1);
        let binary_result = (value as i16) - 1;

        cpu.reg_status.carry = (binary_result & 0xff00) != 0;
        
        cpu.memory.write_at(&self.addr_result.value, &[new_value as u8]);
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for DcpInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::instr::debug_fmt("dcp", &self.addr_result))
    }
}