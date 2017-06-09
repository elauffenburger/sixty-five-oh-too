use super::Cpu;
use cpu::ProcessorStatusRegister;
use super::InstrResult;

use std::fmt;

pub fn clc(cpu: &mut Cpu) -> Box<InstrResult> {
    let mut reg_status = cpu.reg_status.clone();
    reg_status.carry = false;

    clear("clc", reg_status)
}

pub fn cld(cpu: &mut Cpu) -> Box<InstrResult> {
    let mut reg_status = cpu.reg_status.clone();
    reg_status.decimal_mode = false;

    clear("cld", reg_status)
}

pub fn cli(cpu: &mut Cpu) -> Box<InstrResult> {
    let mut reg_status = cpu.reg_status.clone();
    reg_status.irq_disable = false;

    clear("cli", reg_status)
}

pub fn clv(cpu: &mut Cpu) -> Box<InstrResult> {
    let mut reg_status = cpu.reg_status.clone();
    reg_status.overflow = false;

    clear("clv", reg_status)
}

fn clear(instr_name: &'static str, reg_status: ProcessorStatusRegister) -> Box<InstrResult> {
    Box::new(ClearInstrResult {
        bytes: 1,
        cycles: 2,
        new_reg_status: reg_status,
        instr_name: instr_name
    })
}

struct ClearInstrResult {
    bytes: u8,
    cycles: u8,
    new_reg_status: ProcessorStatusRegister,
    instr_name: &'static str
}

impl InstrResult for ClearInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        cpu.reg_status = self.new_reg_status.clone()
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for ClearInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::debug_fmt(self.instr_name, &super::addr::implicit()))
    }
}