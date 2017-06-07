use super::Cpu;
use cpu::ProcessorStatusRegister;
use super::InstrResult;

pub fn clc(cpu: &mut Cpu) -> Box<InstrResult> {
    let mut reg_status = cpu.reg_status.clone();
    reg_status.carry = false;

    clear(reg_status)
}

pub fn cld(cpu: &mut Cpu) -> Box<InstrResult> {
    let mut reg_status = cpu.reg_status.clone();
    reg_status.decimal_mode = false;

    clear(reg_status)
}

pub fn cli(cpu: &mut Cpu) -> Box<InstrResult> {
    let mut reg_status = cpu.reg_status.clone();
    reg_status.irq_disable = false;

    clear(reg_status)
}

pub fn clv(cpu: &mut Cpu) -> Box<InstrResult> {
    let mut reg_status = cpu.reg_status.clone();
    reg_status.overflow = false;

    clear(reg_status)
}

fn clear(reg_status: ProcessorStatusRegister) -> Box<InstrResult> {
    Box::new(ClearInstrResult {
        bytes: 1,
        cycles: 2,
        new_reg_status: reg_status,
    })
}

struct ClearInstrResult {
    bytes: u8,
    cycles: u8,
    new_reg_status: ProcessorStatusRegister,
}

impl InstrResult for ClearInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        cpu.reg_status = self.new_reg_status.clone()
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}
