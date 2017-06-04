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
use std::fmt;

pub trait InstrResult : fmt::Debug {
    fn run(&self, cpu: &mut Cpu) -> ();
    fn get_num_cycles(&self) -> u8;
}

pub fn debug_fmt(instr_name: &'static str, addr_result: &addr::AddrResult) -> String {
    format!("{}, {:?}", instr_name, addr_result)
}