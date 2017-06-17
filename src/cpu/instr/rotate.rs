use super::InstrResult;
use super::Cpu;
use super::super::addr;
use util;
use std::fmt;

pub mod rol {
    use super::InstrResult;
    use super::Cpu;
    use super::addr;
    use super::Direction;

    #[allow(unused_variables)]
    pub fn acc(cpu: &mut Cpu) -> Box<InstrResult> {
        rol(addr::acc(cpu), 1, 2)
    }

    pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page(cpu);

        rol(addr_result, 2, 5)
    }

    pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page_x(cpu);

        rol(addr_result, 2, 6)
    }

    pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs(cpu);

        rol(addr_result, 3, 6)
    }

    pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs_x(cpu);

        rol(addr_result, 3, 7)
    }

    pub fn rol(addr_result: addr::AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
        super::rotate("rol", addr_result, Direction::Left, bytes, cycles)
    }
}

pub mod ror {
    use super::InstrResult;
    use super::Cpu;
    use super::addr;
    use super::Direction;

    #[allow(unused_variables)]
    pub fn acc(cpu: &mut Cpu) -> Box<InstrResult> {
        ror(addr::acc(cpu), 1, 2)
    }

    pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page(cpu);

        ror(addr_result, 2, 5)
    }

    pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page_x(cpu);

        ror(addr_result, 2, 6)
    }

    pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs(cpu);

        ror(addr_result, 3, 6)
    }

    pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs_x(cpu);

        ror(addr_result, 3, 7)
    }

    fn ror(addr_result: addr::AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
        super::rotate("ror", addr_result, Direction::Right, bytes, cycles)
    }
}

enum Direction {
    Left,
    Right,
}

#[allow(unused_variables)]
fn rotate(instr_name: &'static str, addr_result: addr::AddrResult, direction: Direction, bytes: u8, cycles: u8) -> Box<InstrResult> {
    Box::new(RotateInstrResult {
        bytes: bytes,
        cycles: cycles,
        direction: direction,
        instr_name: instr_name,
        addr_result: addr_result,
    })
}

struct RotateInstrResult {
    bytes: u8,
    cycles: u8,
    direction: Direction,
    instr_name: &'static str,
    addr_result: addr::AddrResult,
}

impl InstrResult for RotateInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        let old_value = self.addr_result.resolve(cpu) as i8;

        let new_value = match self.direction {
            Direction::Left => {
                let new_carry_bit = util::test_bit_set(old_value as u8, 7);

                let result = old_value << 1;
                util::set_bit(old_value as u8, 0, cpu.reg_status.carry);

                cpu.reg_status.carry = new_carry_bit;

                result
            }
            Direction::Right => {
                let new_carry_bit = util::test_bit_set(old_value as u8, 0);

                let result = old_value >> 1;
                util::set_bit(old_value as u8, 7, cpu.reg_status.carry);

                cpu.reg_status.carry = new_carry_bit;

                result
            }
        };

        match self.addr_result.addr_mode {
            addr::AddrMode::Accumulator => cpu.reg_acc = new_value,
            _ => cpu.memory.write_at(&self.addr_result.value, &[new_value as u8]),
        };

        cpu.reg_status.negative = util::test_bit_set(new_value as u8, 7);
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for RotateInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::debug_fmt(self.instr_name, &addr::implicit()))
    }
}