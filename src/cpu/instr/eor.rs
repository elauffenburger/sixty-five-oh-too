use super::Cpu;
use super::InstrResult;
use super::super::addr;
use self::addr::AddrResult;

pub fn imm(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::imm(cpu);

    eor(cpu, addr_result, 2, 2)
}

pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::zero_page(cpu);

    eor(cpu, addr_result, 2, 3)
}

pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::zero_page_x(cpu);

    eor(cpu, addr_result, 2, 4)
}

pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs(cpu);

    eor(cpu, addr_result, 3, 4)
}

pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs_x(cpu);

    eor(cpu, addr_result, 3, 4)
}

pub fn abs_y(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs_y(cpu);

    eor(cpu, addr_result, 3, 4)
}

pub fn ind_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::ind_x(cpu);

    eor(cpu, addr_result, 2, 6)
}

pub fn ind_y(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::ind_y(cpu);

    eor(cpu, addr_result, 2, 5)
}

fn eor(cpu: &mut Cpu, addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
    let result = cpu.reg_acc & addr_result.value as i8;

    let final_cycles = match addr_result.crosses_boundary.unwrap_or(false) {
        true => cycles + 1,
        false => cycles
    };

    Box::new(EorInstrResult {
        bytes: bytes,
        cycles: final_cycles,
        result: result
    })
}

struct EorInstrResult {
    bytes: u8,
    cycles: u8,
    result: i8
}

impl InstrResult for EorInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        cpu.reg_acc = self.result;

        cpu.reg_status.zero = self.result == 0;
        cpu.reg_status.negative = self.result < 0;
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}