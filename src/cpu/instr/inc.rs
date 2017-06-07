use cpu;
use super::Cpu;
use super::super::addr;
use super::InstrResult;

enum IncrementType {
    Register(cpu::Register),
    Memory(u16),
}

pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
    let address = addr::zero_page(cpu).value;

    inc(cpu, IncrementType::Memory(address), 2, 5)
}

pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let address = addr::zero_page_x(cpu).value;

    inc(cpu, IncrementType::Memory(address), 2, 6)
}

pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
    let address = addr::abs(cpu).value;

    inc(cpu, IncrementType::Memory(address), 3, 6)
}

pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let address = addr::abs_x(cpu).value;

    inc(cpu, IncrementType::Memory(address), 3, 6)
}

pub fn dex(cpu: &mut Cpu) -> Box<InstrResult> {
    inc(cpu, IncrementType::Register(cpu::Register::X), 1, 2)
}

pub fn dey(cpu: &mut Cpu) -> Box<InstrResult> {
    inc(cpu, IncrementType::Register(cpu::Register::Y), 1, 2)
}

fn inc(cpu: &mut Cpu, inc_type: IncrementType, bytes: u8, cycles: u8) -> Box<InstrResult> {
    let result = match &inc_type {
        &IncrementType::Register(ref reg) => {
            match reg {
                &cpu::Register::X => cpu.reg_x + 1,
                &cpu::Register::Y => cpu.reg_y + 1,
                _ => panic!("unsupported cpu::Register value!"),
            }
        }
        &IncrementType::Memory(ref address) => {
            let val = cpu.memory.read_u8_at(address) as i8;

            val + 1
        }
    };

    Box::new(IncInstrResult {
        bytes: bytes,
        cycles: cycles,
        increment_type: inc_type,
        result: result,
    })
}

struct IncInstrResult {
    bytes: u8,
    cycles: u8,
    increment_type: IncrementType,
    result: i8,
}

impl InstrResult for IncInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        match &self.increment_type {
            &IncrementType::Register(ref reg) => {
                match reg {
                    &cpu::Register::X => cpu.reg_x = self.result,
                    &cpu::Register::Y => cpu.reg_y = self.result,
                    _ => panic!("unsupported cpu::Register type!"),
                }
            }
            &IncrementType::Memory(ref address) => cpu.memory.write_at(&address, &[self.result as u8]),
        }

        cpu.reg_status.zero = self.result == 0;
        cpu.reg_status.negative = self.result < 0;
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}
