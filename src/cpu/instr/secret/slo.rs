use super::addr;
use super::instr;
use super::cpu;

use std::fmt;

pub fn zero_page(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    slo(addr::zero_page(cpu), 2, 5)
}

pub fn zero_page_x(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    slo(addr::zero_page_x(cpu), 2, 6)
}

pub fn abs(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    slo(addr::abs(cpu), 3, 6)
}

pub fn abs_x(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    slo(addr::abs_x(cpu), 3, 7)
}

pub fn abs_y(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    slo(addr::abs_y(cpu), 3, 7)
}

pub fn ind_x(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    slo(addr::ind_x(cpu), 2, 8)
}

pub fn ind_y(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    slo(addr::ind_y(cpu), 2, 8)
}

fn slo(addr_result: addr::AddrResult, bytes: u8, cycles: u8) -> Box<instr::InstrResult> {
    Box::new(SloInstrResult {
        bytes: bytes,
        cycles: cycles,
        addr_result: addr_result
    })
}

struct SloInstrResult {
    bytes: u8,
    cycles: u8,
    addr_result: addr::AddrResult
}

impl instr::InstrResult for SloInstrResult {
    fn run(&self, cpu: &mut cpu::Cpu) {
        let mem_value = self.addr_result.resolve(cpu) as i8;

        let new_mem_value = mem_value << 1;
        let mut binary_result: i16 = (mem_value as i16) << 1;

        let final_value = cpu.reg_acc | new_mem_value;
        binary_result = (cpu.reg_acc as i16) | (new_mem_value as i16);

        cpu.reg_status.negative = final_value < 0;
        cpu.reg_status.zero = final_value == 0;
        cpu.reg_status.carry = (binary_result & 0xff00) != 0;
        
        cpu.reg_acc = final_value;
        cpu.memory.write_at(&self.addr_result.value, &[new_mem_value as u8]);
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for SloInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::instr::debug_fmt("slo", &self.addr_result))
    }
}