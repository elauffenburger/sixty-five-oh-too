use super::InstrResult;
use cpu::Cpu;
use std::fmt;

#[allow(unused_variables)]
pub fn imp(cpu: &mut Cpu) -> Box<InstrResult> {
    Box::new(NopInstrResult {})
}

struct NopInstrResult {}

impl InstrResult for NopInstrResult {
    #[allow(unused_variables)]
    fn run(&self, cpu: &mut Cpu) {}

    fn get_num_cycles(&self) -> u8 {
        2
    }
}

impl fmt::Debug for NopInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::debug_fmt("nop", &super::addr::implicit()))
    }
}