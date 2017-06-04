use super::super::addr;
use super::super::addr::{ AddrResult };
use super::{ InstrResult };
use cpu::Cpu;

#[allow(unused_variables)]
pub fn acc(cpu: &mut Cpu) -> Box<InstrResult> {
    lsr(&addr::implicit(), 1, 2, true)
}

pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::zero_page(cpu);

    lsr(&res, 2, 5, false)
}

pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::zero_page_x(cpu);

    lsr(&res, 2, 6, false)
}

pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::abs(cpu);

    lsr(&res, 3, 6, false)
}

pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::abs_x(cpu);

    lsr(&res, 3, 7, false)
}

fn lsr(addr_result: &AddrResult, bytes: u8, cycles: u8, is_acc: bool) -> Box<InstrResult> {
    Box::new(LsrResult {
        bytes: bytes,
        cycles: cycles,
        is_acc: is_acc,
        address: addr_result.value
    })
}

struct LsrResult {
    bytes: u8,
    cycles: u8,
    is_acc: bool,
    address: u16
}

impl InstrResult for LsrResult {
    fn run(&self, cpu: &mut Cpu) {
        let original_value = match self.is_acc {
            true => cpu.reg_acc as u8,
            _ => cpu.memory.read_u8_at(&self.address)
        };

        let new_value = original_value >> 0x01;

        cpu.reg_status.zero = new_value == 0;
        cpu.reg_status.carry = (original_value & 0b0000_0001) == 1;
        cpu.reg_status.negative = ((new_value & 0b1000_0000) >> 7) == 1;

        match self.is_acc {
            true => cpu.reg_acc = new_value as i8,
            _ => cpu.memory.write_at(&self.address, &[new_value])
        }
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}