use super::InstrResult;
use super::Cpu;
use super::super::addr;
use self::addr::AddrResult;
use cpu;
use util;

pub mod rol {
    use super::InstrResult;
    use super::Cpu;
    use super::cpu;
    use super::addr;
    use super::AddrResult;
    use super::Source;
    use super::Direction;

    pub fn acc(cpu: &mut Cpu) -> Box<InstrResult> {
        rol(cpu, Source::Acc, 1, 2)
    }

    pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page(cpu);

        rol(cpu, Source::Mem(addr_result.value), 2, 5)
    }

    pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page_x(cpu);

        rol(cpu, Source::Mem(addr_result.value), 2, 6)
    }

    pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs(cpu);

        rol(cpu, Source::Mem(addr_result.value), 3, 6)
    }

    pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs_x(cpu);

        rol(cpu, Source::Mem(addr_result.value), 3, 7)
    }

    fn rol(cpu: &mut Cpu, source: Source, bytes: u8, cycles: u8) -> Box<InstrResult> {
        super::rotate(cpu, Direction::Left, source, bytes, cycles)
    }
}

pub mod ror {
    use super::InstrResult;
    use super::Cpu;
    use super::cpu;
    use super::addr;
    use super::AddrResult;
    use super::Source;
    use super::Direction;

    pub fn acc(cpu: &mut Cpu) -> Box<InstrResult> {
        ror(cpu, Source::Acc, 1, 2)
    }

    pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page(cpu);

        ror(cpu, Source::Mem(addr_result.value), 2, 5)
    }

    pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page_x(cpu);

        ror(cpu, Source::Mem(addr_result.value), 2, 6)
    }

    pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs(cpu);

        ror(cpu, Source::Mem(addr_result.value), 3, 6)
    }

    pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs_x(cpu);

        ror(cpu, Source::Mem(addr_result.value), 3, 7)
    }

    fn ror(cpu: &mut Cpu, source: Source, bytes: u8, cycles: u8) -> Box<InstrResult> {
        super::rotate(cpu, Direction::Right, source, bytes, cycles)
    }
}

enum Direction {
    Left,
    Right
}

enum Source {
    Acc,
    Mem(u16)
}

fn rotate(cpu: &mut Cpu, direction: Direction, to_rotate: Source, bytes: u8, cycles: u8) -> Box<InstrResult> {
    Box::new(RotateInstrResult {
        bytes: bytes,
        cycles: cycles,
        direction: direction, 
        to_rotate: to_rotate
    })
}

struct RotateInstrResult {
    bytes: u8,
    cycles: u8,
    direction: Direction,
    to_rotate: Source
}

impl InstrResult for RotateInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        let old_value = match self.to_rotate {
            Source::Acc => cpu.reg_acc as u8,
            Source::Mem(ref address) => {
                cpu.memory.read_u8_at(address)
            }
        };

        let new_value = match self.direction {
            Direction::Left => {
                let new_carry_bit = util::test_bit_set(old_value, 7);

                let mut result = old_value << 1;
                util::set_bit(old_value as u8, 0, cpu.reg_status.carry);

                cpu.reg_status.carry = new_carry_bit; 

                result
            },
            Direction::Right => {
                let new_carry_bit = util::test_bit_set(old_value, 0);

                let mut result = old_value >> 1;
                util::set_bit(old_value as u8, 7, cpu.reg_status.carry);

                cpu.reg_status.carry = new_carry_bit; 

                result
            }
        };

        match self.to_rotate {
            Source::Acc => cpu.reg_acc = new_value as i8,
            Source::Mem(ref address) => cpu.memory.write_at(address, &[new_value])
        };

        cpu.reg_status.negative = util::test_bit_set(new_value, 7);        
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}