use super::Cpu;
use super::super::addr;
use self::addr::AddrResult;
use super::InstrResult;

fn pha(cpu: &mut Cpu) -> Box<InstrResult> {
    let acc = cpu.reg_acc;

    push(cpu, acc as u8, 1, 3)
}

fn php(cpu: &mut Cpu) -> Box<InstrResult> {
    let status: u8 = cpu.reg_status.clone().into();

    push(cpu, status, 1, 3)
}

fn push(cpu: &mut Cpu, to_push: u8, bytes: u8, cycles: u8) -> Box<InstrResult> {
    Box::new(PushInstrResult {
        bytes: bytes,
        cycles: cycles,
        to_push: to_push
    })
}

struct PushInstrResult {
    bytes: u8,
    cycles: u8,
    to_push: u8
}

impl InstrResult for PushInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        cpu.push_u8(self.to_push);
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}