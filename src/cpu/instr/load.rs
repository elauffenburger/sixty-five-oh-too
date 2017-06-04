use super::super::super::cpu;
use super::Cpu;
use super::InstrResult;
use super::super::addr;
use self::addr::AddrResult;

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
        super::load(cpu::Register::A, addr_result, bytes, cycles)
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
        super::load(cpu::Register::X, addr_result, bytes, cycles)
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
        super::load(cpu::Register::Y, addr_result, bytes, cycles)
    }
}

fn load(register: cpu::Register, addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
    let total_cycles = match addr_result.crosses_boundary.unwrap_or(false) {
        true => cycles + 1,
        false => cycles
    };

    Box::new(LoadInstrResult {
        bytes: bytes,
        cycles: total_cycles,
        value: addr_result.value as i8,
        register: register
    })
}

struct LoadInstrResult {
    bytes: u8,
    cycles: u8,
    value: i8,
    register: cpu::Register
}

impl InstrResult for LoadInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        match self.register {
            cpu::Register::A => cpu.reg_acc = self.value,
            cpu::Register::X => cpu.reg_x = self.value,
            cpu::Register::Y => cpu.reg_y = self.value
        }

        cpu.reg_status.negative = self.value < 0;
        cpu.reg_status.zero = self.value == 0;
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}