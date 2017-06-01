use super::Cpu;
use super::InstrResult;

pub fn resolve(opcode: u8) -> Option<fn(&mut Cpu) -> Box<InstrResult>> {
    match opcode {
        0x29 => Some(super::and::imm),
        0x25 => Some(super::and::zero_page),
        0x35 => Some(super::and::zero_page_x),
        0x2d => Some(super::and::abs),
        0x3d => Some(super::and::abs_x),
        0x39 => Some(super::and::abs_y),
        0x21 => Some(super::and::ind_x),
        0x31 => Some(super::and::ind_y),
        0x0a => Some(super::asl::acc),
        0x06 => Some(super::asl::zero_page),
        0x16 => Some(super::asl::zero_page_x),
        0x0e => Some(super::asl::abs),
        0x1e => Some(super::asl::abs_x),
        0x90 => Some(super::branch::bcc),
        0xb0 => Some(super::branch::bcs),
        0xf0 => Some(super::branch::beq),
        0x24 => Some(super::bit::zero_page),
        0x2c => Some(super::bit::abs),
        0x30 => Some(super::branch::bmi),
        0xd0 => Some(super::branch::bne),
        0x10 => Some(super::branch::bpl),
        0x00 => Some(super::brk::brk),
        0x50 => Some(super::branch::bvc),
        0x70 => Some(super::branch::bvs),
        _ => None
    }
}