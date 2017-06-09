use super::Cpu;
use super::InstrResult;
use std::fmt;

fn pha(cpu: &mut Cpu) -> Box<InstrResult> {
    let acc = cpu.reg_acc;

    push("pha", acc as u8, 1, 3)
}

fn php(cpu: &mut Cpu) -> Box<InstrResult> {
    let status: u8 = cpu.reg_status.clone().into();

    push("php", status, 1, 3)
}

fn push(instr_name: &'static str, to_push: u8, bytes: u8, cycles: u8) -> Box<InstrResult> {
    Box::new(PushInstrResult {
        bytes: bytes,
        cycles: cycles,
        to_push: to_push,
        instr_name: instr_name
    })
}

struct PushInstrResult {
    bytes: u8,
    cycles: u8,
    to_push: u8,
    instr_name: &'static str
}

impl InstrResult for PushInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        cpu.push_u8(self.to_push);
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for PushInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::debug_fmt(self.instr_name, &super::addr::implicit()))
    }
}