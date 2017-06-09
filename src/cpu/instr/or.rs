use super::Cpu;
use super::InstrResult;
use super::super::addr;
use self::addr::AddrResult;

use std::fmt;

enum OrType {
    LogicalExclusive,
    LogicalInclusive,
}

pub mod eor {
    use super::Cpu;
    use super::InstrResult;
    use super::addr;
    use super::AddrResult;

    pub fn imm(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::imm(cpu);

        eor(cpu, addr_result, 2, 2)
    }

    pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page(cpu);

        eor(cpu, addr_result, 2, 3)
    }

    pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page_x(cpu);

        eor(cpu, addr_result, 2, 4)
    }

    pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs(cpu);

        eor(cpu, addr_result, 3, 4)
    }

    pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs_x(cpu);

        eor(cpu, addr_result, 3, 4)
    }

    pub fn abs_y(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs_y(cpu);

        eor(cpu, addr_result, 3, 4)
    }

    pub fn ind_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::ind_x(cpu);

        eor(cpu, addr_result, 2, 6)
    }

    pub fn ind_y(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::ind_y(cpu);

        eor(cpu, addr_result, 2, 5)
    }

    pub fn eor(cpu: &mut Cpu, addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
        super::or("eor", cpu, super::OrType::LogicalExclusive, addr_result, bytes, cycles)
    }

    #[cfg(test)]
    mod test {
        use super::Cpu;

        #[test]
        fn eor() {
            let mut cpu = Cpu::new();
            cpu.reg_pc = 0x1000;
            cpu.reg_acc = 0x64;
            cpu.memory.mem[0x1000] = 0x52;

            let instr = super::imm(&mut cpu);
            instr.run(&mut cpu);

            assert_eq!(0x64 ^ 0x52, cpu.reg_acc);
        }
    }
}

pub mod ora {
    use super::Cpu;
    use super::InstrResult;
    use super::addr;
    use super::AddrResult;

    pub fn imm(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::imm(cpu);

        ora(cpu, addr_result, 2, 2)
    }

    pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page(cpu);

        ora(cpu, addr_result, 2, 3)
    }

    pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page_x(cpu);

        ora(cpu, addr_result, 2, 4)
    }

    pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs(cpu);

        ora(cpu, addr_result, 3, 4)
    }

    pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs_x(cpu);

        ora(cpu, addr_result, 3, 4)
    }

    pub fn abs_y(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs_y(cpu);

        ora(cpu, addr_result, 3, 4)
    }

    pub fn ind_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::ind_x(cpu);

        ora(cpu, addr_result, 2, 6)
    }

    pub fn ind_y(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::ind_y(cpu);

        ora(cpu, addr_result, 2, 5)
    }

    pub fn ora(cpu: &mut Cpu, addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
        super::or("ora", cpu, super::OrType::LogicalInclusive, addr_result, bytes, cycles)
    }

    #[cfg(test)]
    mod test {
        use super::Cpu;

        #[test]
        fn ora() {
            let mut cpu = Cpu::new();
            cpu.reg_pc = 0x1000;
            cpu.reg_acc = 0x64;
            cpu.memory.mem[0x1000] = 0x52;

            let instr = super::imm(&mut cpu);
            instr.run(&mut cpu);

            assert_eq!(0x64 | 0x52, cpu.reg_acc);
        }
    }
}

fn or(instr_name: &'static str, cpu: &mut Cpu, or_type: OrType, addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
    let final_cycles = match addr_result.crosses_boundary.unwrap_or(false) {
        true => cycles + 1,
        false => cycles,
    };

    Box::new(OrInstrResult {
        bytes: bytes,
        cycles: final_cycles,
        or_type: or_type,
        addr_result: addr_result,
        instr_name: instr_name
    })
}

struct OrInstrResult {
    bytes: u8,
    cycles: u8,
    or_type: OrType,
    addr_result: AddrResult,
    instr_name: &'static str
}

impl InstrResult for OrInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        let value = self.addr_result.resolve(cpu) as i8;

        let result = match self.or_type {
            OrType::LogicalExclusive => cpu.reg_acc ^ value,
            OrType::LogicalInclusive => cpu.reg_acc | value,
        };

        cpu.reg_acc = result;

        cpu.reg_status.zero = result == 0;
        cpu.reg_status.negative = result < 0;
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for OrInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::debug_fmt(self.instr_name, &self.addr_result))
    }
}