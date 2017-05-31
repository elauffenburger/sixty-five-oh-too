pub mod adc;
pub mod and;
pub mod asl;
pub mod bit;
pub mod branch;
pub mod brk;

use super::Cpu;

pub trait InstrResult {
    fn run(&self, cpu: &mut Cpu) -> ();
    fn get_num_cycles(&self) -> u8;
}