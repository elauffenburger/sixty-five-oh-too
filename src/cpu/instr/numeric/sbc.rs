use super::super::Cpu;
use super::InstrResult;
use super::addr;
use super::addr::AddrResult;
use std::fmt;
use super::{ Operation, numeric };

pub fn imm(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::imm(cpu);

    sbc(addr_result, 2, 2)
}

pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::zero_page(cpu);

    sbc(addr_result, 2, 3)
}

pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::zero_page_x(cpu);

    sbc(addr_result, 2, 4)
}

pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs(cpu);

    sbc(addr_result, 3, 4)
}

pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs_x(cpu);

    sbc(addr_result, 3, 4)
}

pub fn abs_y(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs_y(cpu);

    sbc(addr_result, 3, 4)
}

pub fn ind_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::ind_x(cpu);

    sbc(addr_result, 2, 6)
}

pub fn ind_y(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::ind_y(cpu);

    sbc(addr_result, 2, 5)
}

fn sbc(addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
    super::numeric("sbc", addr_result, Operation::Sub, bytes, cycles)
}
