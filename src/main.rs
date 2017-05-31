#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod cpu;
mod asm;
mod util;

fn main() {
    let cpu = cpu::Cpu::new();
}
