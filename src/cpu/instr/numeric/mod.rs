pub mod adc;
pub mod sbc;

use super::super::Cpu;
use super::InstrResult;
use super::addr;
use super::addr::AddrResult;
use std::fmt;

enum Operation {
    Add,
    Sub,
}

fn numeric(instr_name: &'static str, addr_result: AddrResult, operation: Operation, bytes: u8, cycles: u8) -> Box<InstrResult> {
    let total_cycles = match addr_result.crosses_boundary.unwrap_or(false) {
        true => cycles + 1,
        false => cycles,
    };

    Box::new(NumericInstrResult {
        bytes: bytes,
        cycles: total_cycles,
        operation: operation,
        instr_name: instr_name,
        addr_result: addr_result
    })
}

struct NumericInstrResult {
    bytes: u8,
    cycles: u8,
    operation: Operation,
    instr_name: &'static str,
    addr_result: AddrResult
}

impl InstrResult for NumericInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        let value = self.addr_result.value as i8;

        let (value, carry_bit_value) = match self.operation {
            Operation::Add => (value, 1i8),
            Operation::Sub => (-value, -1i8),
        };

        let mut binary_result = (cpu.reg_acc as u8 as u16) + (value as u8 as u16);

        let (mut result, mut overflowing) = cpu.reg_acc.overflowing_add(value);
        if cpu.reg_status.carry {
            binary_result += carry_bit_value as u8 as u16;

            let (add_carry_result, add_carry_overflowing) = result.overflowing_add(carry_bit_value);

            result = add_carry_result;
            overflowing = add_carry_overflowing || overflowing;
        }

        cpu.reg_acc = result;
        cpu.reg_status.carry = (binary_result & 0xff00) != 0;
        cpu.reg_status.overflow = overflowing;
        cpu.reg_status.negative = result < 0;
        cpu.reg_status.zero = result == 0;
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for NumericInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::debug_fmt(self.instr_name, &self.addr_result))
    }
}
