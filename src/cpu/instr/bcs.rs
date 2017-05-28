use super::Cpu;
use super::super::addr;
use super::super::addr::AddrResult;
use super::InstrResult;

pub fn rel(cpu: &mut Cpu) -> AddrResult {
    let res = addr::rel(cpu);
    bcs(cpu, &res, 2, 2)
}

pub fn bcs(cpu: &mut Cpu, addr_result: &AddrResult, bytes: u8, cycles: u8) -> InstrResult {
    let mut final_cycles = cycles;
    let should_branch = cpu.reg_status.carry;
    
    if should_branch {
        final_cycles += 1;
    }

    if addr_result.crosses_boundary {
        final_cycles += 2;
    }

    BcsResult {
        cycles: final_cycles,
        next_pc = should_branch ? addr_result.value : cpu.reg_pc + 1
    }
}

struct BcsResult {
    cycles: u8,
    next_pc: u16
}

impl InstrResult for BcsResult {
    fn run(&self, cpu: &mut Cpu) {
        cpu.reg_pc = self.next_pc;
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}
