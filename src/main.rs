#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod cpu;
mod asm;
mod util;

fn main() {
    let mut cpu = cpu::Cpu::new();

    cpu.run();
}

#[cfg(test)]
pub mod test {
    #[test]
    fn basic_program() {
        let program = &[// LDA #$01
                        0xa9,
                        0x01,
                        // STA $0200
                        0x8d,
                        0x00,
                        0x02,
                        // LDA #$05
                        0xa9,
                        0x05,
                        // STA $0201
                        0x8d,
                        0x01,
                        0x02];

        let mut cpu = super::cpu::Cpu::new();
        cpu.load_program(0x6000, program);

        cpu.run();
    }
}
