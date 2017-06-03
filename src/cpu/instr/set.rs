use cpu::Cpu;
use super::InstrResult;

pub fn sec(cpu: &mut Cpu) -> Box<InstrResult> {
    set(Flag::Carry)
}

enum Flag {
    Carry,
    Decimal,
    InterruptDisable
}

fn set(to_set: Flag) -> Box<InstrResult> {
    Box::new(SetInstrResult {
        to_set: to_set
    })
}

struct SetInstrResult {
    to_set: Flag
}

impl InstrResult for SetInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        match self.to_set {
            Flag::Carry => cpu.reg_status.carry = true,
            Flag::Decimal => cpu.reg_status.decimal_mode = true,
            Flag::InterruptDisable => cpu.reg_status.irq_disable = true
        }
    }

    fn get_num_cycles(&self) -> u8 {
        2
    }
}