use cpu;
use super::Cpu;
use super::InstrResult;

enum PullDestination {
    Accumulator,
    Status,
}

#[allow(unused_variables)]
fn pla(cpu: &mut Cpu) -> Box<InstrResult> {
    pull(PullDestination::Accumulator, 1, 3)
}

#[allow(unused_variables)]
fn plp(cpu: &mut Cpu) -> Box<InstrResult> {
    pull(PullDestination::Status, 1, 3)
}

fn pull(pull_dest: PullDestination, bytes: u8, cycles: u8) -> Box<InstrResult> {
    Box::new(PullInstrResult {
        bytes: bytes,
        cycles: cycles,
        pull_dest: pull_dest,
    })
}

struct PullInstrResult {
    bytes: u8,
    cycles: u8,
    pull_dest: PullDestination,
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
