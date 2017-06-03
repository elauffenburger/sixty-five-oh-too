use super::Cpu;
use super::InstrResult;

#[allow(unused_variables)]
pub fn brk(cpu: &mut Cpu) -> Box<InstrResult> {
    Box::new(BrkResult {
        bytes: 1,
        cycles: 7
    })
}

struct BrkResult {
    bytes: u8,
    cycles: u8
}

impl InstrResult for BrkResult {
    fn run(&self, cpu: &mut Cpu) {
        let pc = cpu.reg_pc;
        let pc_hi = ((pc & 0xff00) >> 2) as u8;
        let pc_lo = (pc & 0x00ff) as u8;
        
        let status: u8 = cpu.reg_status.clone().into();

        cpu.push_u8(pc_hi);
        cpu.push_u8(pc_lo);
        cpu.push_u8(status);

        let irq_vec = cpu.memory.read_u16_at(&0xfffe);
        cpu.reg_pc = irq_vec;

        cpu.reg_status.brk = true;
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

#[cfg(test)]
mod test {
    use super::Cpu;

    #[test]
    fn brk() {
        let mut cpu = Cpu::new();
        cpu.reg_pc = 0x1234;

        let pc_hi = ((cpu.reg_pc & 0xff00) >> 2) as u8;
        let pc_lo = (cpu.reg_pc & 0x00ff) as u8;

        cpu.reg_status.carry = true;
        cpu.reg_status.negative = true;

        let status_reg: u8 = cpu.reg_status.clone().into();
        assert_eq!(status_reg, 0xa1);

        let brk_instr = super::brk(&mut cpu);
        brk_instr.run(&mut cpu);

        assert_eq!(cpu.reg_sp, 0xfc);
        assert_eq!(cpu.pop_u8().unwrap(), status_reg);
        assert_eq!(cpu.pop_u8().unwrap(), pc_lo);
        assert_eq!(cpu.pop_u8().unwrap(), pc_hi);
    }
}