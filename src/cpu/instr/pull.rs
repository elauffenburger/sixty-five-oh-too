use cpu;
use super::Cpu;
use super::InstrResult;
use std::fmt;

enum PullDestination {
    Accumulator,
    Status,
}

#[allow(unused_variables)]
fn pla(cpu: &mut Cpu) -> Box<InstrResult> {
    pull("pla", PullDestination::Accumulator, 1, 3)
}

#[allow(unused_variables)]
fn plp(cpu: &mut Cpu) -> Box<InstrResult> {
    pull("plp", PullDestination::Status, 1, 3)
}

fn pull(instr_name: &'static str, pull_dest: PullDestination, bytes: u8, cycles: u8) -> Box<InstrResult> {
    Box::new(PullInstrResult {
        bytes: bytes,
        cycles: cycles,
        pull_dest: pull_dest,
        instr_name: instr_name
    })
}

struct PullInstrResult {
    bytes: u8,
    cycles: u8,
    pull_dest: PullDestination,
    instr_name: &'static str
}

impl InstrResult for PullInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        let value = cpu.pop_u8().unwrap();

        match self.pull_dest {
            PullDestination::Accumulator => cpu.reg_acc = value as i8,
            PullDestination::Status => {
                cpu.reg_status = cpu::ProcessorStatusRegister::from(value);
            }
        }
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for PullInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::debug_fmt(self.instr_name, &super::addr::implicit()))
    }
}