use super::super::addr;
use super::super::addr::{ AddrResult };
use super::{ InstrResult };
use cpu::Cpu;

pub fn imm(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::imm(cpu);

    and(cpu, &res, 2, 2)
}

pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::zero_page(cpu);

    and(cpu, &res, 2, 3)
}

pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::zero_page_x(cpu);

    and(cpu, &res, 2, 3)
}

pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::abs(cpu);

    and(cpu, &res, 2, 3)
}

pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::abs_x(cpu);

    and(cpu, &res, 2, 4)
}

pub fn abs_y(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::abs_y(cpu);

    and(cpu, &res, 3, 4)
}

pub fn ind_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::abs_x(cpu);

    and(cpu, &res, 3, 4)
}

pub fn ind_y(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::abs_y(cpu);

    and(cpu, &res, 2, 4)
}

fn and(cpu: &mut Cpu, addr_result: &AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
    let imm = cpu.memory.read_u8_at(&addr_result.value);
    let result = cpu.reg_acc & imm;

    let final_cycles = match addr_result.crosses_boundary.unwrap_or(false) {
        true => cycles + 1,
        _ => cycles
    };

    Box::new(AndResult {
        bytes: bytes,
        cycles: final_cycles,
        result: result
    })
}

struct AndResult {
    bytes: u8,
    cycles: u8,
    result: u8
}

impl InstrResult for AndResult {
    fn run(&self, cpu: &mut Cpu) {
        cpu.reg_status.zero = self.result == 0;
        cpu.reg_status.negative = (self.result as i8) < 0;

        cpu.reg_acc = self.result;
    }
    
    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

#[cfg(test)]
mod test {
    use cpu::Cpu;

    #[test]
    fn and() {
        let mut cpu = Cpu::new();
        cpu.memory.mem[0xfe] = 0x01;
        cpu.memory.mem[0x01] = 0x0f;
        cpu.reg_acc = 0xff;
        cpu.reg_pc = 0xfe;

        let result = super::imm(&mut cpu);
        result.run(&mut cpu);

        assert_eq!(cpu.reg_acc, 0x0f);
    }
}