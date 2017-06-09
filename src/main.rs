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
    use std;
    use std::io;
    use std::io::{ Read, Seek };
    use super::cpu;

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

        let mut cpu = cpu::Cpu::new();
        cpu.load_program(0x6000, program);

        cpu.run();
    }

    #[test]
    fn nestest() {
        let mut file_reader = std::io::BufReader::new(std::fs::File::open("assets/nestest.nes").unwrap());

        let mut file_bytes: Vec<u8> = Vec::new();

        // skip header data then populate vec
        file_reader.seek(io::SeekFrom::Start(16));
        file_reader.take(0x4000).read_to_end(&mut file_bytes);

        let mut cpu = cpu::Cpu::new();
        cpu.load_program(0xc000, &file_bytes);
        
        cpu.run();
    }
}
