use cpu;
use super::Cpu;
use super::super::addr;
use super::InstrResult;
use std::fmt;

enum DecrementType {
    Register(cpu::Register),
    Memory(addr::AddrResult),
}

pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::zero_page(cpu);

    dec(cpu, DecrementType::Memory(addr_result), 2, 5)
}

pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::zero_page_x(cpu);

    dec(cpu, DecrementType::Memory(addr_result), 2, 6)
}

pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs(cpu);

    dec(cpu, DecrementType::Memory(addr_result), 3, 6)
}

pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs_x(cpu);

    dec(cpu, DecrementType::Memory(addr_result), 3, 6)
}

pub fn dex(cpu: &mut Cpu) -> Box<InstrResult> {
    dec(cpu, DecrementType::Register(cpu::Register::X), 1, 2)
}

pub fn dey(cpu: &mut Cpu) -> Box<InstrResult> {
    dec(cpu, DecrementType::Register(cpu::Register::Y), 1, 2)
}

fn dec(cpu: &mut Cpu, dec_type: DecrementType, bytes: u8, cycles: u8) -> Box<InstrResult> {
    Box::new(DecInstrResult {
        bytes: bytes,
        cycles: cycles,
        decrement_type: dec_type,
    })
}

struct DecInstrResult {
    bytes: u8,
    cycles: u8,
    decrement_type: DecrementType,
}

impl InstrResult for DecInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        let result = match &self.decrement_type {
            &DecrementType::Register(ref reg) => {
                match reg {
                    &cpu::Register::X => cpu.reg_x - 1,
                    &cpu::Register::Y => cpu.reg_y - 1,
                    _ => panic!("unsupported cpu::Register value!"),
                }
            }
            &DecrementType::Memory(ref addr_result) => {
                let val = cpu.memory.read_u8_at(&addr_result.value) as i8;

                val - 1
            }
        };

        match &self.decrement_type {
            &DecrementType::Register(ref reg) => {
                match reg {
                    &cpu::Register::X => cpu.reg_x = result,
                    &cpu::Register::Y => cpu.reg_y = result,
                    _ => panic!("unsupported cpu::Register type!"),
                }
            }
            &DecrementType::Memory(ref addr_result) => cpu.memory.write_at(&addr_result.value, &[result as u8]),
        }

        cpu.reg_status.zero = result == 0;
        cpu.reg_status.negative = result < 0;
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for DecInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let implicit = addr::implicit();

        let (instr_name, ref addr_result) = match &self.decrement_type {
            &DecrementType::Memory(ref addr_result) => {
                ("dec", addr_result)
            },
            &DecrementType::Register(ref register) => {
                match register {
                    &cpu::Register::X => ("dex", &implicit),
                    &cpu::Register::Y => ("dey", &implicit),
                    _ => panic!("unsupported cpu::Register type!")
                }
            }
        };

        write!(f, "{}", super::debug_fmt(instr_name, addr_result))
    }
}
