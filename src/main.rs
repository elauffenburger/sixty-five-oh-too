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
        let parser = asm::Parser::new();
    }
}