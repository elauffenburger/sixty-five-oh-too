use super::Cpu;
use super::super::addr;
use self::addr::AddrResult;
use super::InstrResult;
use std::fmt;

pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs(cpu);

    jmp(addr_result, 3, 3)
}

pub fn ind(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::ind(cpu);

    jmp(addr_result, 3, 5)
}

fn jmp(addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
    Box::new(JmpInstrResult {
        bytes: bytes,
        cycles: cycles,
        addr_result: addr_result
    })
}

struct JmpInstrResult {
    bytes: u8,
    cycles: u8,
    addr_result: AddrResult
}

impl InstrResult for JmpInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        cpu.reg_pc = self.addr_result.value
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for JmpInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::debug_fmt("jmp", &self.addr_result))
    }
}