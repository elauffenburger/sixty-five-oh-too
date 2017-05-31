use super::Cpu;
use super::super::addr;
use super::super::addr::AddrResult;
use super::InstrResult;

pub fn bcc(cpu: &mut Cpu) -> Box<InstrResult> {
    let should_branch = cpu.reg_status.carry == false;
    branch(cpu, should_branch, 2, 2)
}

pub fn bcs(cpu: &mut Cpu) -> Box<InstrResult> {
    let should_branch = cpu.reg_status.carry;
    branch(cpu, should_branch, 2, 2)
}

pub fn beq(cpu: &mut Cpu) -> Box<InstrResult> {
    let should_branch = cpu.reg_status.zero;
    branch(cpu, should_branch, 2, 2)
}

pub fn bmi(cpu: &mut Cpu) -> Box<InstrResult> {
    let should_branch = cpu.reg_status.negative;
    branch(cpu, should_branch, 2, 2)
}

pub fn bne(cpu: &mut Cpu) -> Box<InstrResult> {
    let should_branch = !cpu.reg_status.zero;
    branch(cpu, should_branch, 2, 2)
}

pub fn bpl(cpu: &mut Cpu) -> Box<InstrResult> {
    let should_branch = !cpu.reg_status.negative;
    branch(cpu, should_branch, 2, 2)
}

pub fn bvc(cpu: &mut Cpu) -> Box<InstrResult> {
    let should_branch = !cpu.reg_status.overflow;
    branch(cpu, should_branch, 2, 2)
}

pub fn bvs(cpu: &mut Cpu) -> Box<InstrResult> {
    let should_branch = cpu.reg_status.overflow;
    branch(cpu, should_branch, 2, 2)
}

fn branch(cpu: &mut Cpu, should_branch: bool, bytes: u8, cycles: u8) -> Box<InstrResult> {
    let addr_result = addr::rel(cpu);
    let mut final_cycles = cycles;
    
    if should_branch {
        final_cycles += 1;
    }

    if addr_result.crosses_boundary.unwrap_or(false) {
        final_cycles += 2;
    }

    let next_pc = match should_branch {
        true => addr_result.value,
        false => cpu.reg_pc
    };

    Box::new(BranchResult {
        cycles: final_cycles,
        next_pc: next_pc
    })
}

struct BranchResult {
    cycles: u8,
    next_pc: u16
}

impl InstrResult for BranchResult {
    fn run(&self, cpu: &mut Cpu) {
        cpu.reg_pc = self.next_pc;
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

#[cfg(test)]
mod test {
    use super::Cpu;

    #[test]
    fn bcc() {
        let mut cpu = Cpu::new();
        cpu.reg_pc = 0x01;
        cpu.memory.mem[0x01] = 0x38;

        cpu.reg_status.carry = false;
        let should_branch_res = super::bcc(&mut cpu);
        should_branch_res.run(&mut cpu);

        assert_eq!(cpu.reg_pc, 0x3a);

        cpu.reg_pc = 0x01;
        cpu.memory.mem[0x01] = 0xff;

        cpu.reg_status.carry = true;
        let should_not_branch_res = super::bcc(&mut cpu);
        should_not_branch_res.run(&mut cpu);

        assert_eq!(cpu.reg_pc, 0x2);
    }

    #[test]
    fn bcs() {

    }

    #[test]
    fn beq() {

    }
}