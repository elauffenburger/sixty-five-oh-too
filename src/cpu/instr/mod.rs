pub mod adc;
pub mod and;
pub mod asl;

use super::Cpu;

pub trait InstrResult {
    fn run(&self, cpu: &mut Cpu) -> ();
}