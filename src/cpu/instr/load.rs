use super::super::super::cpu;
use super::Cpu;
use super::InstrResult;
use super::super::addr;
use self::addr::AddrResult;

use std::fmt;

pub mod lda {
    use super::cpu;
    use super::Cpu;
    use super::InstrResult;
    use super::addr;
    use self::addr::AddrResult;

    pub fn imm(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::imm(cpu);

        lda(addr_result, 2, 2)
    }

    pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page(cpu);

        lda(addr_result, 2, 3)
    }

    pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page_x(cpu);

        lda(addr_result, 2, 4)
    }

    pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs(cpu);

        lda(addr_result, 3, 4)
    }

    pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs_x(cpu);

        lda(addr_result, 3, 4)
    }

    pub fn abs_y(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs_y(cpu);

        lda(addr_result, 3, 4)
    }

    pub fn ind_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::ind_x(cpu);

        lda(addr_result, 2, 6)
    }

    pub fn ind_y(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::ind_y(cpu);

        lda(addr_result, 2, 5)
    }

    fn lda(addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
        super::load("lda", cpu::Register::A, addr_result, bytes, cycles)
    }
}
pub mod ldx {
    use super::cpu;
    use super::Cpu;
    use super::InstrResult;
    use super::addr;
    use self::addr::AddrResult;

    pub fn imm(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::imm(cpu);

        ldx(addr_result, 2, 2)
    }

    pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page(cpu);

        ldx(addr_result, 2, 3)
    }

    pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page_x(cpu);

        ldx(addr_result, 2, 4)
    }

    pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs(cpu);

        ldx(addr_result, 3, 4)
    }

    pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs_x(cpu);

        ldx(addr_result, 3, 4)
    }

    fn ldx(addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
        super::load("ldx", cpu::Register::X, addr_result, bytes, cycles)
    }
}

pub mod ldy {
    use super::cpu;
    use super::Cpu;
    use super::InstrResult;
    use super::addr;
    use self::addr::AddrResult;

    pub fn imm(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::imm(cpu);

        ldy(addr_result, 2, 2)
    }

    pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page(cpu);

        ldy(addr_result, 2, 3)
    }

    pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page_x(cpu);

        ldy(addr_result, 2, 4)
    }

    pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs(cpu);

        ldy(addr_result, 3, 4)
    }

    pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs_x(cpu);

        ldy(addr_result, 3, 4)
    }

    fn ldy(addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
        super::load("ldy", cpu::Register::Y, addr_result, bytes, cycles)
    }
}

fn load(instr_name: &'static str, register: cpu::Register, addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
    let total_cycles = match addr_result.crosses_boundary.unwrap_or(false) {
        true => cycles + 1,
        false => cycles,
    };

    Box::new(LoadInstrResult {
        bytes: bytes,
        cycles: total_cycles,
        addr_result: addr_result,
        register: register,
        instr_name: instr_name,
    })
}

struct LoadInstrResult {
    bytes: u8,
    cycles: u8,
    addr_result: AddrResult,
    register: cpu::Register,
    instr_name: &'static str,
}

impl InstrResult for LoadInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        let value = self.addr_result.resolve(cpu) as i8;

        match self.register {
            cpu::Register::A => cpu.reg_acc = value,
            cpu::Register::X => cpu.reg_x = value,
            cpu::Register::Y => cpu.reg_y = value,
        }

        cpu.reg_status.negative = value < 0;
        cpu.reg_status.zero = value == 0;
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for LoadInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               super::debug_fmt(self.instr_name, &self.addr_result))
    }
}
