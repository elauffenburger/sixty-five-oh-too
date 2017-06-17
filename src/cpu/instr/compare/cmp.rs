use super::InstrResult;
use super::AddrResult;
use super::addr;
use super::Cpu;

pub fn imm(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::imm(cpu);

    cmp(cpu, addr_result, 2, 2)
}

pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::zero_page(cpu);

    cmp(cpu, addr_result, 2, 3)
}

pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::zero_page_x(cpu);

    cmp(cpu, addr_result, 3, 4)
}

pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs(cpu);

    cmp(cpu, addr_result, 3, 4)
}

pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs_x(cpu);

    cmp(cpu, addr_result, 3, 4)
}

pub fn abs_y(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs_y(cpu);

    cmp(cpu, addr_result, 3, 4)
}

pub fn ind_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::ind_x(cpu);

    cmp(cpu, addr_result, 2, 6)
}

pub fn ind_y(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::ind_y(cpu);

    cmp(cpu, addr_result, 2, 5)
}

pub fn cmp(cpu: &mut Cpu, addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
    let reg_val = cpu.reg_acc as i8;

    super::compare("cmp", addr_result, reg_val, bytes, cycles)
}