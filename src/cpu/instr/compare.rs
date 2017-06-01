use super::InstrResult;
use super::super::addr;
use self::addr::AddrResult;
use super::Cpu;

pub mod cmp {
    use super::InstrResult;
    use super::AddrResult;
    use super::compare;
    use super::addr;
    use super::Cpu;

    pub fn imm(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::imm(cpu);

        cmp(cpu, addr_result, 2, 2)
    }

    pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page(cpu);

        cmp(cpu, addr_result, 2, 3)
    }

    pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page_x(cpu);

        cmp(cpu, addr_result, 3, 4)
    }

    pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs(cpu);

        cmp(cpu, addr_result, 3, 4)
    }

    pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs_x(cpu);

        cmp(cpu, addr_result, 3, 4)
    }

    pub fn abs_y(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs_y(cpu);

        cmp(cpu, addr_result, 3, 4)
    }

    pub fn ind_x(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::ind_x(cpu);

        cmp(cpu, addr_result, 2, 6)
    }

    pub fn ind_y(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::ind_y(cpu);

        cmp(cpu, addr_result, 2, 5)
    }

    fn cmp(cpu: &mut Cpu, addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
        let reg_val = cpu.reg_acc as i8;

        compare(cpu, addr_result, reg_val, 2, 2)
    }
}

pub mod cpx {
    use super::InstrResult;
    use super::AddrResult;
    use super::compare;
    use super::addr;
    use super::Cpu;

    pub fn imm(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::imm(cpu);

        cpx(cpu, addr_result, 2, 2)
    }

    pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page(cpu);

        cpx(cpu, addr_result, 2, 3)
    }

    pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs(cpu);

        cpx(cpu, addr_result, 3, 4)
    }

    fn cpx(cpu: &mut Cpu, addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
        let reg_val = cpu.reg_x as i8;

        compare(cpu, addr_result, reg_val, 2, 2)
    }
}

pub mod cpy {
    use super::InstrResult;
    use super::AddrResult;
    use super::compare;
    use super::addr;
    use super::Cpu;

    pub fn imm(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::imm(cpu);

        cpy(cpu, addr_result, 2, 2)
    }

    pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::zero_page(cpu);

        cpy(cpu, addr_result, 2, 3)
    }

    pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
        let addr_result = addr::abs(cpu);

        cpy(cpu, addr_result, 3, 4)
    }

    fn cpy(cpu: &mut Cpu, addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
        let reg_val = cpu.reg_y as i8;

        compare(cpu, addr_result, reg_val, 2, 2)
    }
}

fn compare(cpu: &mut Cpu, addr_result: AddrResult, reg_val: i8, bytes: u8, cycles: u8) -> Box<InstrResult> {
    let total_cycles = match addr_result.crosses_boundary.unwrap_or(false) { 
        true => cycles + 1, 
        false => cycles
    };

    Box::new(CompareInstrResult {
        bytes: bytes,
        cycles: total_cycles,
        reg_val: reg_val,
        mem_val: addr_result.value as i8
    })
}

struct CompareInstrResult {
    bytes: u8,
    cycles: u8,
    reg_val: i8,
    mem_val: i8
}

impl InstrResult for CompareInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        let result = self.reg_val - self.mem_val;

        cpu.reg_status.carry = self.reg_val >= self.mem_val;
        cpu.reg_status.zero = self.reg_val == self.mem_val;
        cpu.reg_status.negative = result < 0;
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}