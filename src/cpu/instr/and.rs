use super::super::addr;
use cpu::Cpu;

pub fn imm(cpu: &mut Cpu) {
    let res = addr::imm(cpu);

    and(cpu, &res.result)
}

pub fn zero_page(cpu: &mut Cpu) {
    let res = addr::imm(cpu);

    and(cpu, &res.result)
}

pub fn zero_page_x(cpu: &mut Cpu) {
    let res = addr::imm(cpu);

    and(cpu, &res.result)
}

pub fn zero_page_y(cpu: &mut Cpu) {
    let res = addr::imm(cpu);

    and(cpu, &res.result)
}

fn and(cpu: &mut Cpu, imm: &u8) {
    let result = cpu.reg_acc & imm;

    cpu.reg_status.zero = result == 0;
    cpu.reg_status.negative = result < 0;

    cpu.reg_acc = result;
}

#[cfg(test)]
mod test {
    use cpu::Cpu;

    #[test]
    fn test_and_imm() {
        let mut cpu = Cpu::new();
        cpu.memory.mem[0xFE] = 0b0000_1111;
        cpu.reg_acc = 0xff;
        cpu.reg_pc = 0xFE;

        super::imm(&mut cpu);

        assert!(cpu.reg_acc == 0b0000_1111);
    }
}