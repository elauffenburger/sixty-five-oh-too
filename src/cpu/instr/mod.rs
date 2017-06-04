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
pub mod transfer;

pub mod resolver;

use super::Cpu;
use super::addr;

pub trait InstrResult {
    fn run(&self, cpu: &mut Cpu) -> ();
    fn get_num_cycles(&self) -> u8;
}

pub fn print(instr_name: &'static str, addr_result: &addr::AddrResult) {
    println!("{}, {:?}", instr_name, addr_result);
}