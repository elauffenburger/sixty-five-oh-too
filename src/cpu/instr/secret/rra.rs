use super::addr;
use super::instr;
use super::cpu;

use super::instr::dec;
use super::instr::compare::cmp;

use std::fmt;

pub fn zero_page(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    rra(addr::zero_page(cpu), 2, 5)
}

pub fn zero_page_x(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    rra(addr::zero_page_x(cpu), 2, 6)
}

pub fn abs(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    rra(addr::abs(cpu), 3, 6)
}

pub fn abs_x(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    rra(addr::abs_x(cpu), 3, 7)
}

pub fn abs_y(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    rra(addr::abs_y(cpu), 3, 7)
}

pub fn ind_x(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    rra(addr::ind_x(cpu), 2, 8)
}

pub fn ind_y(cpu: &mut cpu::Cpu) -> Box<instr::InstrResult> {
    rra(addr::ind_y(cpu), 2, 8)
}

fn rra(addr_result: addr::AddrResult, bytes: u8, cycles: u8) -> Box<instr::InstrResult> {
    Box::new(RraInstrResult {
        bytes: bytes,
        cycles: cycles,
        addr_result: addr_result,
    })
}

struct RraInstrResult {
    bytes: u8,
    cycles: u8,
    addr_result: addr::AddrResult,
}

impl instr::InstrResult for RraInstrResult {
    fn run(&self, cpu: &mut cpu::Cpu) {
        let ror = super::instr::rotate::ror::ror(self.addr_result.clone(), 0, 0);
        let adc = super::instr::adc::adc(self.addr_result.clone(), 0, 0);

        (*ror).run(cpu);
        (*adc).run(cpu);
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for RraInstrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::instr::debug_fmt("rra", &self.addr_result))
    }
}

#[cfg(test)]
mod test {
    use super::cpu;

    #[test]
    fn ror_adc() {
        let mut cpu: cpu::Cpu = cpu::Cpu::new();
        cpu.reset();

        let program = &[
            // lda #$37
            0xa9, 0x37,
            
            // sta $0647
            0x8d, 0x47, 0x06,

            // lda #$75
            0xa9, 0x75,

            // ror $0647
            0x6e, 0x47, 0x06,

            // adc $0647
            0x6d, 0x47, 0x06
        ];

        cpu.load_program(0x6000, program);

        cpu.run();

        assert_eq!(cpu.reg_acc as u8, 0x11);
    }

    #[test]
    fn rra() {
        let mut cpu: cpu::Cpu = cpu::Cpu::new();
        cpu.reset();

        let program = &[
            // lda #$37
            0xa9, 0x37,
            
            // sta $0647
            0x8d, 0x47, 0x06,

            // lda #$75
            0xa9, 0x75,

            // rra $0647
            0x7b, 0x47, 0x06,
        ];

        cpu.load_program(0x6000, program);

        cpu.run();

        assert_eq!(cpu.reg_acc as u8, 0x11);
    }
}