use super::super::cpu;

pub struct AddrResult<T> {
    pub result: T,
    pub bytes: u8,
    pub cycles: u8,
}

pub fn imm(cpu: &mut cpu::Cpu) -> AddrResult<u8> {
    AddrResult {
        result: cpu.memory.read_u8_at(&cpu.reg_pc),
        bytes: 2,
        cycles: 2
    }
}

pub fn zero_page(cpu: &mut cpu::Cpu) -> AddrResult<u8>  {
    AddrResult {
        result: cpu.memory.deref_u8_at(&cpu.reg_pc),
        bytes: 2,
        cycles: 3
    }
}

pub fn zero_page_x(cpu: &mut cpu::Cpu) -> AddrResult<u8> {
    let addr = cpu.memory.read_u8_at(&cpu.reg_pc);
    let imm = cpu.memory.read_u8_at(&((cpu.reg_x + addr) as u16));

    AddrResult {
       result: imm,
       bytes: 2,
       cycles: 4
    }
}