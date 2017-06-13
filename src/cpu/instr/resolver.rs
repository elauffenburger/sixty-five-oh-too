use super::Cpu;
use super::InstrResult;

pub fn resolve(opcode: u8) -> Option<fn(&mut Cpu) -> Box<InstrResult>> {
    match opcode {
        0x69 => Some(super::adc::imm),
        0x65 => Some(super::adc::zero_page),
        0x75 => Some(super::adc::zero_page_x),
        0x6d => Some(super::adc::abs),
        0x7d => Some(super::adc::abs_x),
        0x79 => Some(super::adc::abs_y),
        0x61 => Some(super::adc::ind_x),
        0x71 => Some(super::adc::ind_y),
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
        0x18 => Some(super::clear::clc),
        0xd8 => Some(super::clear::cld),
        0x58 => Some(super::clear::cli),
        0xb8 => Some(super::clear::clv),
        0xc9 => Some(super::compare::cmp::imm),
        0xc5 => Some(super::compare::cmp::zero_page),
        0xd5 => Some(super::compare::cmp::zero_page_x),
        0xcd => Some(super::compare::cmp::abs),
        0xdd => Some(super::compare::cmp::abs_x),
        0xd9 => Some(super::compare::cmp::abs_y),
        0xc1 => Some(super::compare::cmp::ind_y),
        0xd1 => Some(super::compare::cmp::ind_x),
        0xe0 => Some(super::compare::cpx::imm),
        0xe4 => Some(super::compare::cpx::zero_page),
        0xec => Some(super::compare::cpx::abs),
        0xc0 => Some(super::compare::cpy::imm),
        0xc4 => Some(super::compare::cpy::zero_page),
        0xcc => Some(super::compare::cpy::abs),
        0xc6 => Some(super::dec::zero_page),
        0xd6 => Some(super::dec::zero_page_x),
        0xce => Some(super::dec::abs),
        0xde => Some(super::dec::abs_x),
        0xca => Some(super::dec::dex),
        0x88 => Some(super::dec::dey),
        0x49 => Some(super::or::eor::imm),
        0x45 => Some(super::or::eor::zero_page),
        0x55 => Some(super::or::eor::zero_page_x),
        0x4d => Some(super::or::eor::abs),
        0x5d => Some(super::or::eor::abs_x),
        0x59 => Some(super::or::eor::abs_y),
        0x41 => Some(super::or::eor::ind_x),
        0x51 => Some(super::or::eor::ind_y),
        0xe6 => Some(super::inc::zero_page),
        0xf6 => Some(super::inc::zero_page_x),
        0xee => Some(super::inc::abs),
        0xfe => Some(super::inc::abs_x),
        0xe8 => Some(super::inc::inx),
        0xc8 => Some(super::inc::iny),
        0x4c => Some(super::jmp::abs),
        0x6c => Some(super::jmp::ind),
        0x20 => Some(super::jsr::jsr),
        0xa9 => Some(super::load::lda::imm),
        0xa5 => Some(super::load::lda::zero_page),
        0xb5 => Some(super::load::lda::zero_page_x),
        0xad => Some(super::load::lda::abs),
        0xbd => Some(super::load::lda::abs_x),
        0xb9 => Some(super::load::lda::abs_y),
        0xa1 => Some(super::load::lda::ind_x),
        0xb1 => Some(super::load::lda::ind_y),
        0xa2 => Some(super::load::ldx::imm),
        0xa6 => Some(super::load::ldx::zero_page),
        0xb6 => Some(super::load::ldx::zero_page_y),
        0xae => Some(super::load::ldx::abs),
        0xbe => Some(super::load::ldx::abs_y),
        0xa0 => Some(super::load::ldy::imm),
        0xa4 => Some(super::load::ldy::zero_page),
        0xb4 => Some(super::load::ldy::zero_page_x),
        0xac => Some(super::load::ldy::abs),
        0xbc => Some(super::load::ldy::abs_x),
        0x4a => Some(super::lsr::acc),
        0x46 => Some(super::lsr::zero_page),
        0x56 => Some(super::lsr::zero_page_x),
        0x4e => Some(super::lsr::abs),
        0x5e => Some(super::lsr::abs_x),
        0xea => Some(super::nop::imp),
        0x1a => Some(super::nop::imp),
        0x3a => Some(super::nop::imp),
        0x5a => Some(super::nop::imp),
        0x7a => Some(super::nop::imp),
        0xda => Some(super::nop::imp),
        0xfa => Some(super::nop::imp),
        0x09 => Some(super::or::ora::imm),
        0x05 => Some(super::or::ora::zero_page),
        0x15 => Some(super::or::ora::zero_page_x),
        0x0d => Some(super::or::ora::abs),
        0x1d => Some(super::or::ora::abs_x),
        0x19 => Some(super::or::ora::abs_y),
        0x01 => Some(super::or::ora::ind_x),
        0x11 => Some(super::or::ora::ind_y),
        0x48 => Some(super::push::pha),
        0x08 => Some(super::push::php),
        0x68 => Some(super::pull::pla),
        0x28 => Some(super::pull::plp),
        0x2a => Some(super::rotate::rol::acc),
        0x26 => Some(super::rotate::rol::zero_page),
        0x36 => Some(super::rotate::rol::zero_page_x),
        0x2e => Some(super::rotate::rol::abs),
        0x3e => Some(super::rotate::rol::abs_x),
        0x6a => Some(super::rotate::ror::acc),
        0x66 => Some(super::rotate::ror::zero_page),
        0x76 => Some(super::rotate::ror::zero_page_x),
        0x6e => Some(super::rotate::ror::abs),
        0x7e => Some(super::rotate::ror::abs_x),
        0x40 => Some(super::ret::rti),
        0x60 => Some(super::ret::rts),
        0xe9 => Some(super::numeric::sbc::imm),
        0xe5 => Some(super::numeric::sbc::zero_page),
        0xf5 => Some(super::numeric::sbc::zero_page_x),
        0xed => Some(super::numeric::sbc::abs),
        0xfd => Some(super::numeric::sbc::abs_x),
        0xf9 => Some(super::numeric::sbc::abs_y),
        0xe1 => Some(super::numeric::sbc::ind_x),
        0xf1 => Some(super::numeric::sbc::ind_y),
        0x04 => Some(super::secret::dop::zero_page),
        0x14 => Some(super::secret::dop::zero_page_x),
        0x34 => Some(super::secret::dop::zero_page_x),
        0x44 => Some(super::secret::dop::zero_page),
        0x54 => Some(super::secret::dop::zero_page_x),
        0x64 => Some(super::secret::dop::zero_page),
        0x74 => Some(super::secret::dop::zero_page_x),
        0x80 => Some(super::secret::dop::imm),
        0x82 => Some(super::secret::dop::imm),
        0x89 => Some(super::secret::dop::imm),
        0xc2 => Some(super::secret::dop::imm),
        0xd4 => Some(super::secret::dop::zero_page_x),
        0xe2 => Some(super::secret::dop::imm),
        0xf4 => Some(super::secret::dop::zero_page_x),
        0x0c => Some(super::secret::top::abs),
        0x1c => Some(super::secret::top::abs_x),
        0x3c => Some(super::secret::top::abs_x),
        0x5c => Some(super::secret::top::abs_x),
        0x7c => Some(super::secret::top::abs_x),
        0xdc => Some(super::secret::top::abs_x),
        0xfc => Some(super::secret::top::abs_x),
        0x38 => Some(super::set::sec),
        0xf8 => Some(super::set::sed),
        0x78 => Some(super::set::sei),
        0x85 => Some(super::store::sta::zero_page),
        0x95 => Some(super::store::sta::zero_page_x),
        0x8d => Some(super::store::sta::abs),
        0x9d => Some(super::store::sta::abs_x),
        0x99 => Some(super::store::sta::abs_y),
        0x81 => Some(super::store::sta::ind_x),
        0x91 => Some(super::store::sta::ind_y),
        0x86 => Some(super::store::stx::zero_page),
        0x96 => Some(super::store::stx::zero_page_y),
        0x8e => Some(super::store::stx::abs),
        0x84 => Some(super::store::sty::zero_page),
        0x94 => Some(super::store::sty::zero_page_x),
        0x8c => Some(super::store::sty::abs),
        0xaa => Some(super::transfer::tay),
        0xa8 => Some(super::transfer::tay),
        0xba => Some(super::transfer::tsx),
        0x8a => Some(super::transfer::txa),
        0x9a => Some(super::transfer::txs),
        0x98 => Some(super::transfer::tya),
        _ => None,
    }
}
