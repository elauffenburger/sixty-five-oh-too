use super::InstrResult;
use super::AddrResult;
use super::addr;
use super::Cpu;

pub fn imm(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::imm(cpu);

    cpx(cpu, addr_result, 2, 2)
}

pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::zero_page(cpu);

    cpx(cpu, addr_result, 2, 3)
}

pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs(cpu);

    cpx(cpu, addr_result, 3, 4)
}

fn cpx(cpu: &mut Cpu, addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
    let reg_val = cpu.reg_x as i8;

    super::compare("cpx", addr_result, reg_val, bytes, cycles)
}