pub mod adc;
pub mod and;

use super::Cpu;

pub trait InstrResult {
    fn run(&self, cpu: &mut Cpu) -> ();
}