use super::Cpu;
use super::super::addr;
use super::InstrResult;

use std::fmt;

pub fn jsr(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs(cpu);

    Box::new(JsrInstrResult {
        bytes: 3,
        cycles: 6,
        addr_result: addr_result,
    })
}

struct JsrInstrResult {
    bytes: u8,
    cycles: u8,
    addr_result: addr::AddrResult
}

impl InstrResult for JsrInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        let rts_addr = cpu.reg_pc - 0x01;

        let pc_hi = ((rts_addr & 0xff00) >> 8) as u8;
        let pc_lo = (rts_addr & 0x00ff) as u8;

        cpu.push_u8(pc_hi);
        cpu.push_u8(pc_lo);

        cpu.reg_pc = self.addr_result.value;
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for JsrInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::debug_fmt("jsr", &self.addr_result))
    }
}