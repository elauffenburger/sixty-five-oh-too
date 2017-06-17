use cpu;
use super::Cpu;
use super::super::addr;
use super::InstrResult;
use std::fmt;

pub enum IncrementType {
    Register(cpu::Register),
    Memory(addr::AddrResult),
}

pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::zero_page(cpu);

    inc(IncrementType::Memory(addr_result), 2, 5)
}

pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::zero_page_x(cpu);

    inc(IncrementType::Memory(addr_result), 2, 6)
}

pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs(cpu);

    inc(IncrementType::Memory(addr_result), 3, 6)
}

pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs_x(cpu);

    inc(IncrementType::Memory(addr_result), 3, 6)
}

#[allow(unused_variables)]
pub fn inx(cpu: &mut Cpu) -> Box<InstrResult> {
    inc(IncrementType::Register(cpu::Register::X), 1, 2)
}

#[allow(unused_variables)]
pub fn iny(cpu: &mut Cpu) -> Box<InstrResult> {
    inc(IncrementType::Register(cpu::Register::Y), 1, 2)
}

pub fn inc(inc_type: IncrementType, bytes: u8, cycles: u8) -> Box<InstrResult> {
    Box::new(IncInstrResult {
        bytes: bytes,
        cycles: cycles,
        increment_type: inc_type,
    })
}

struct IncInstrResult {
    bytes: u8,
    cycles: u8,
    increment_type: IncrementType,
}

impl InstrResult for IncInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        let result = match &self.increment_type {
            &IncrementType::Register(ref reg) => {
                match reg {
                    &cpu::Register::X => cpu.reg_x.overflowing_add(1).0,
                    &cpu::Register::Y => cpu.reg_y.overflowing_add(1).0,
                    _ => panic!("unsupported cpu::Register value!"),
                }
            }
            &IncrementType::Memory(ref addr_result) => {
                let val = cpu.memory.read_u8_at(&addr_result.value) as i8;

                val.overflowing_add(1).0
            }
        };

        match &self.increment_type {
            &IncrementType::Register(ref reg) => {
                match reg {
                    &cpu::Register::X => cpu.reg_x = result,
                    &cpu::Register::Y => cpu.reg_y = result,
                    _ => panic!("unsupported cpu::Register type!"),
                }
            }
            &IncrementType::Memory(ref addr_result) => cpu.memory.write_at(&addr_result.value, &[result as u8]),
        }

        cpu.reg_status.zero = result == 0;
        cpu.reg_status.negative = result < 0;
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for IncInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let implicit = addr::implicit();

        let (instr_name, ref addr_result) = match &self.increment_type {
            &IncrementType::Memory(ref addr_result) => {
                ("inc", addr_result)
            },
            &IncrementType::Register(ref register) => {
                match register {
                    &cpu::Register::X => ("inx", &implicit),
                    &cpu::Register::Y => ("iny", &implicit),
                    _ => panic!("unsupported cpu::Register type!")
                }
            }
        };

        write!(f, "{}", super::debug_fmt(instr_name, addr_result))
    }
}