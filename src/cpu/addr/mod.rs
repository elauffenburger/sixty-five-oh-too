#[cfg(test)]
mod test;

use super::Cpu;
use super::mem::MemoryMap;

use std::fmt;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum AddrMode {
    Unknown,
    Implicit,
    Immediate,
    Accumulator,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

pub struct AddrResult {
    pub value: u16,
    pub crosses_boundary: Option<bool>,
    pub addr_mode: AddrMode,
    pub fmt_str: Option<String>,
}

impl AddrResult {
    pub fn resolve(&self, cpu: &mut Cpu) -> u8 {
        match self.addr_mode {
            AddrMode::Immediate | AddrMode::Implicit => self.value as u8,
            AddrMode::Accumulator => cpu.reg_acc as u8,
            AddrMode::Unknown => panic!("unknown addr mode!"),
            _ => cpu.memory.read_u8_at(&self.value),
        }
    }
}

impl fmt::Debug for AddrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value_str = match self.fmt_str {
            Some(ref fmt_str) => fmt_str.clone(),
            None => {
                let result = match self.addr_mode {
                    AddrMode::Immediate => format!("#${:x}", &self.value),
                    AddrMode::Implicit | AddrMode::Accumulator => String::from(""),
                    AddrMode::ZeroPageY | AddrMode::AbsoluteY => format!("${:x},Y", &self.value),
                    AddrMode::ZeroPageX | AddrMode::AbsoluteX => format!("${:x},X", &self.value),
                    AddrMode::IndirectX => format!("(${:x},X)", &self.value),
                    AddrMode::IndirectY => format!("(${:x}),Y", &self.value),
                    _ => format!("${:x}", &self.value), 
                };

                result
            }
        };

        write!(f, "{}", value_str)
    }
}

pub fn implicit() -> AddrResult {
    AddrResult {
        value: 0,
        crosses_boundary: None,
        addr_mode: AddrMode::Implicit,
        fmt_str: None,
    }
}

pub fn acc(cpu: &mut Cpu) -> AddrResult {
    AddrResult {
        value: cpu.reg_acc as i16 as u16,
        crosses_boundary: None,
        addr_mode: AddrMode::Accumulator,
        fmt_str: None,
    }
}

pub fn imm(cpu: &mut Cpu) -> AddrResult {
    AddrResult {
        value: cpu.read_u8() as u16,
        crosses_boundary: None,
        addr_mode: AddrMode::Immediate,
        fmt_str: None,
    }
}

pub fn rel(cpu: &mut Cpu) -> AddrResult {
    // since the pc is incremented by 2 during
    // the course of the instruction decode process,
    // we want to use the initial value
    let initial_pc = cpu.reg_pc - 0x01;

    let addr = (cpu.read_u8() as i8 as i16 + cpu.reg_pc as i16) as u16;

    AddrResult {
        value: addr,
        crosses_boundary: Some(MemoryMap::crosses_page_boundary(&initial_pc, &addr)),
        addr_mode: AddrMode::Relative,
        fmt_str: None,
    }
}

pub fn zero_page(cpu: &mut Cpu) -> AddrResult {
    AddrResult {
        value: cpu.read_u8() as u16,
        crosses_boundary: None,
        addr_mode: AddrMode::ZeroPage,
        fmt_str: None,
    }
}

pub fn zero_page_x(cpu: &mut Cpu) -> AddrResult {
    let addr = (cpu.read_u8() as i8 as i16).overflowing_add(cpu.reg_x as i16).0 as u16;

    // only take least sig byte to simulate zero page wraparound
    let addr_lsb = addr & 0x00ff;

    AddrResult {
        value: addr_lsb,
        crosses_boundary: None,
        addr_mode: AddrMode::ZeroPageX,
        fmt_str: None,
    }
}

pub fn zero_page_y(cpu: &mut Cpu) -> AddrResult {
    let addr = (cpu.read_u8() as i8 as i16).overflowing_add(cpu.reg_y as i16).0 as u16;

    // only take least sig byte to simulate zero page wraparound
    let addr_lsb = addr & 0x00ff;

    AddrResult {
        value: addr_lsb,
        crosses_boundary: None,
        addr_mode: AddrMode::ZeroPageY,
        fmt_str: None,
    }
}

pub fn abs(cpu: &mut Cpu) -> AddrResult {
    AddrResult {
        value: cpu.read_u16(),
        crosses_boundary: None,
        addr_mode: AddrMode::Absolute,
        fmt_str: None,
    }
}

pub fn abs_x(cpu: &mut Cpu) -> AddrResult {
    let partial_addr = cpu.read_u16();
    let offset = cpu.reg_x;

    let addr = (partial_addr as i16).overflowing_add(offset as i16).0 as u16;

    AddrResult {
        value: addr,
        crosses_boundary: Some(MemoryMap::crosses_page_boundary(&partial_addr, &addr)),
        addr_mode: AddrMode::AbsoluteX,
        fmt_str: None,
    }
}

pub fn abs_y(cpu: &mut Cpu) -> AddrResult {
    let partial_addr = cpu.read_u16();
    let offset = cpu.reg_y;

    let addr = partial_addr.overflowing_add(offset as u8 as u16).0 as u16;

    AddrResult {
        value: addr,
        crosses_boundary: Some(MemoryMap::crosses_page_boundary(&partial_addr, &addr)),
        addr_mode: AddrMode::AbsoluteY,
        fmt_str: Some(format!("${:04x},Y -> {:04x}", partial_addr, addr)),
    }
}

pub fn ind(cpu: &mut Cpu) -> AddrResult {
    let indirect_addr = cpu.read_u16();

    // simulate page-boundary bug
    let absolute_addr = match (indirect_addr & 0x00ff) {
        indirect_lo if indirect_lo == 0x00ff => {
            let indirect_hi = indirect_addr & 0xff00;

            let abs_lo = cpu.memory.read_u8_at(&indirect_addr);
            let abs_hi = cpu.memory.read_u8_at(&indirect_hi);

            let result = super::util::to_u16(&[abs_lo, abs_hi]);

            result
        }
        _ => cpu.memory.read_u16_at(&indirect_addr),
    };

    AddrResult {
        value: absolute_addr,
        crosses_boundary: None,
        addr_mode: AddrMode::Indirect,
        fmt_str: None,
    }
}

pub fn ind_x(cpu: &mut Cpu) -> AddrResult {
    // LDA #$05
    // STA $01
    // LDA #$06
    // STA $02
    // LDX #$01
    // LDA ($00,X) ; ($00, X) -> ($00 + X) -> ($01) -> $0605

    // $00
    let base_indirect_addr = cpu.read_u8();

    // $00 + X
    let indirect_addr = base_indirect_addr as i8 as i16 + cpu.reg_x as i16;

    // only take least sig byte to simulate zero page wraparound
    let indirect_lsb = indirect_addr & 0x00ff;

    // ($00 + X) -> $0605
    let addr: u16 = cpu.memory.read_u16_at(&(indirect_lsb as u16));

    AddrResult {
        value: addr,
        crosses_boundary: None,
        addr_mode: AddrMode::IndirectX,
        fmt_str: None,
    }
}

pub fn ind_y(cpu: &mut Cpu) -> AddrResult {
    // LDA #$03
    // STA $01
    // LDA #$07
    // STA $02
    // LDY #$01
    // LDA ($01), Y ; $(01) -> $0703, $0703 + Y -> $0704

    // $01
    let double_indirect = cpu.read_u8();

    // ($01) -> $0703
    let single_indirect = cpu.memory.read_u16_at(&(double_indirect as u16));

    // $0703 + Y
    let addr = (single_indirect as i16).overflowing_add(cpu.reg_y as i16).0 as u16;

    AddrResult {
        value: addr,
        crosses_boundary: Some(MemoryMap::crosses_page_boundary(&single_indirect, &addr)),
        addr_mode: AddrMode::IndirectY,
        fmt_str: None,
    }
}
