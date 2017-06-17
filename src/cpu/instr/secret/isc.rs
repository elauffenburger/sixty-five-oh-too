use super::addr;
use super::instr;
use super::cpu;

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
        let mem_value = self.addr_result.resolve(cpu) as i8;

        let (new_mem_value, new_mem_value_did_overflow) = mem_value.overflowing_add(1);
        let mut binary_result: i16 = (mem_value as i16) + 1;

        let (final_value, final_value_did_overflow) = cpu.reg_acc.overflowing_sub(new_mem_value);
        binary_result = (cpu.reg_acc as i16) - (new_mem_value as i16);

        cpu.reg_status.carry = (binary_result & 0xff00) != 0;
        cpu.reg_status.overflow = new_mem_value_did_overflow | final_value_did_overflow;
        cpu.reg_status.negative = final_value < 0;
        cpu.reg_status.zero = final_value == 0;
        
        cpu.memory.write_at(&self.addr_result.value, &[final_value as u8]);
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