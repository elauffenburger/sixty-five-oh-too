pub mod numeric;
pub use self::numeric::adc;
pub use self::numeric::sbc;
pub mod and;
pub mod asl;
pub mod bit;
pub mod branch;
pub mod brk;
pub mod clear;
pub mod compare;
pub mod dec;
pub mod or;
pub mod inc;
pub mod jmp;
pub mod jsr;
pub mod load;
pub mod lsr;
pub mod nop;
pub mod pull;
pub mod push;
pub mod rotate;
pub mod ret;
pub mod store;

pub mod resolver;

use super::Cpu;

pub trait InstrResult {
    fn run(&self, cpu: &mut Cpu) -> ();
    fn get_num_cycles(&self) -> u8;
}