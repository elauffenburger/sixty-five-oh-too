use super::super::super::cpu;
use super::Cpu;
use super::InstrResult;
use super::super::addr;
use self::addr::AddrResult;

use std::fmt;

pub mod sta {
    use super::cpu;
    use super::Cpu;
    use super::InstrResult;
    use super::addr;
    use self::addr::AddrResult;

    pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page(cpu);

        sta(addr_result, 2, 3)
    }

    pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page_x(cpu);

        sta(addr_result, 2, 4)
    }

    pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs(cpu);

        sta(addr_result, 3, 4)
    }

    pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs_x(cpu);

        sta(addr_result, 3, 5)
    }

    pub fn abs_y(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs_y(cpu);

        sta(addr_result, 3, 5)
    }

    pub fn ind_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::ind_x(cpu);

        sta(addr_result, 2, 6)
    }

    pub fn ind_y(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::ind_y(cpu);

        sta(addr_result, 2, 6)
    }

    fn sta(addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
        super::store("sta", cpu::Register::A, addr_result, bytes, cycles)
    }
}

pub mod stx {
    use super::cpu;
    use super::Cpu;
    use super::InstrResult;
    use super::addr;
    use self::addr::AddrResult;

    pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page(cpu);

        stx(addr_result, 2, 3)
    }

    pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page_x(cpu);

        stx(addr_result, 2, 4)
    }

    pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs(cpu);

        stx(addr_result, 3, 4)
    }

    fn stx(addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
        super::store("stx", cpu::Register::X, addr_result, bytes, cycles)
    }
}

pub mod sty {
    use super::cpu;
    use super::Cpu;
    use super::InstrResult;
    use super::addr;
    use self::addr::AddrResult;

    pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page(cpu);

        sty(addr_result, 2, 3)
    }

    pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page_x(cpu);

        sty(addr_result, 2, 4)
    }

    pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs(cpu);

        sty(addr_result, 3, 4)
    }

    fn sty(addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
        super::store("sty", cpu::Register::Y, addr_result, bytes, cycles)
    }
}

fn store(instr_name: &'static str, register: cpu::Register, addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
    Box::new(StoreInstrResult {
        bytes: bytes,
        cycles: cycles,
        addr_result: addr_result,
        register: register,
        instr_name: instr_name,
    })
}

struct StoreInstrResult {
    bytes: u8,
    cycles: u8,
    addr_result: AddrResult,
    register: cpu::Register,
    instr_name: &'static str,
}

impl InstrResult for StoreInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        let value = match self.register {
            cpu::Register::A => cpu.reg_acc,
            cpu::Register::X => cpu.reg_x,
            cpu::Register::Y => cpu.reg_y,
        };

        cpu.memory.write_at(&self.addr_result.value, &[value as u8]);
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for StoreInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               super::debug_fmt(self.instr_name, &self.addr_result))
    }
}
