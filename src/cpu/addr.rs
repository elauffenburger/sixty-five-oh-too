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
    IndirectY
}

pub struct AddrResult {
    pub value: u16,
    pub crosses_boundary: Option<bool>,
    pub addr_mode: AddrMode
}

impl AddrResult {
    pub fn resolve(&self, cpu: &mut Cpu) -> u8 {
        match self.addr_mode {
            AddrMode::Immediate | AddrMode::Implicit => {
                self.value as u8
            },
            AddrMode::Unknown => {
                panic!("unknown addr mode!")
            },
            _ => cpu.memory.read_u8_at(&self.value)
        }
    }
}

impl fmt::Debug for AddrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value_str = match self.addr_mode {
            AddrMode::Immediate => format!("#${:x}", &self.value),
            AddrMode::Implicit | AddrMode::Accumulator => String::from(""),
            _ => format!("${:x}", &self.value) 
        };

        write!(f, "{}", value_str)
    }
}

pub fn implicit(cpu: &mut Cpu) -> AddrResult {
    AddrResult {
        value: 0,
        crosses_boundary: None,
        addr_mode: AddrMode::Implicit
    }
}

pub fn imm(cpu: &mut Cpu) -> AddrResult {
    AddrResult {
        value: cpu.read_u8() as u16,
        crosses_boundary: None,
        addr_mode: AddrMode::Immediate
    }
}

pub fn rel(cpu: &mut Cpu) -> AddrResult {
    // since the pc is incremented by 2 during
    // the course of the instruction decode process,
    // we want to use the initial value
    let initial_pc = cpu.reg_pc - 0x01;

    let addr = cpu.read_u8() as u16 + cpu.reg_pc;

    AddrResult {
        value: addr,
        crosses_boundary: Some(MemoryMap::crosses_page_boundary(&initial_pc, &addr)),
        addr_mode: AddrMode::Relative
    }
}

pub fn zero_page(cpu: &mut Cpu) -> AddrResult  {
    AddrResult {
        value: cpu.read_u8() as u16,
        crosses_boundary: None,
        addr_mode: AddrMode::ZeroPage
    }
}

pub fn zero_page_x(cpu: &mut Cpu) -> AddrResult {
    let addr = (cpu.read_u8() as u16) + (cpu.reg_x as u16);
    
    // only take least sig byte to simulate zero page wraparound
    let addr_lsb = addr & 0x00ff;

    AddrResult {
       value: addr_lsb,
       crosses_boundary: None,
       addr_mode: AddrMode::ZeroPageX
    }
}

pub fn zero_page_y(cpu: &mut Cpu) -> AddrResult {
    let addr = (cpu.read_u8() as u16) + (cpu.reg_y as u16);
    
    // only take least sig byte to simulate zero page wraparound
    let addr_lsb = addr & 0x00ff;

    AddrResult {
       value: addr_lsb,
       crosses_boundary: None,
       addr_mode: AddrMode::ZeroPageY
    }
}

pub fn abs(cpu: &mut Cpu) -> AddrResult {
    AddrResult {
       value: cpu.read_u16(),
       crosses_boundary: None,
       addr_mode: AddrMode::Absolute
    }
}

pub fn abs_x(cpu: &mut Cpu) -> AddrResult {
    let partial_addr = cpu.read_u16();
    let offset = cpu.reg_x;
    
    let addr = partial_addr + (offset as u16);

    AddrResult {
       value: addr,
       crosses_boundary: Some(MemoryMap::crosses_page_boundary(&partial_addr, &addr)),
       addr_mode: AddrMode::AbsoluteX
    }
}

pub fn abs_y(cpu: &mut Cpu) -> AddrResult {
    let partial_addr = cpu.read_u16();
    let offset = cpu.reg_y;
    
    let addr = partial_addr + (offset as u16);

    AddrResult {
       value: addr,
       crosses_boundary: Some(MemoryMap::crosses_page_boundary(&partial_addr, &addr)),
       addr_mode: AddrMode::AbsoluteY
    }
}

pub fn ind(cpu: &mut Cpu) -> AddrResult {
    let indirect = cpu.read_u16();
    let direct = cpu.memory.read_u16_at(&indirect);

    AddrResult {
        value: direct,
        crosses_boundary: None,
        addr_mode: AddrMode::Indirect
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
    let base_indirect_addr = cpu.read_u8() as i8;

    // $00 + X 
    let indirect_addr = base_indirect_addr as i16 + cpu.reg_x as i16;

    // only take least sig byte to simulate zero page wraparound
    let indirect_lsb = indirect_addr & 0x00ff;

    // ($00 + X) -> $0605
    let addr : u16 = cpu.memory.read_u16_at(&(indirect_lsb as u16));

    AddrResult {
       value: addr,
       crosses_boundary: None,
       addr_mode: AddrMode::IndirectX
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
    let addr = single_indirect + cpu.reg_y as u16;

    AddrResult {
       value: addr,
       crosses_boundary: Some(MemoryMap::crosses_page_boundary(&single_indirect, &addr)),
       addr_mode: AddrMode::IndirectY
    }
}

#[cfg(test)]
mod test {
    use super::Cpu;

    #[test]
    fn imm() {
        let mut cpu = Cpu::new();

        cpu.reg_pc = 0xfe;
        cpu.memory.mem[0xfe] = 0xbe;

        let result = super::imm(&mut cpu);
        assert_eq!(result.value, 0xbe);
    }

    #[test]
    fn rel() {
        let mut cpu = Cpu::new();

        // starting pc: 0xbead
        // ending pc: start + 2 -> 0xbeaf
        // 0xbeef - 0xbeaf = 0x40
        
        // this is is start + 1 (after opcode decode)
        cpu.reg_pc = 0xbeae;
        cpu.memory.mem[0xbeae] = 0x40;

        let result = super::rel(&mut cpu);
        assert_eq!(result.value, 0xbeef);
    }

    #[test]
    fn zero_page() {
        let mut cpu = Cpu::new();

        cpu.reg_pc = 0x0e;
        cpu.memory.mem[0x0e] = 0x05;

        let result = super::zero_page(&mut cpu);
        assert_eq!(result.value, 0x05);
    }

    #[test]
    fn zero_page_x() {
        let mut cpu = Cpu::new();

        cpu.reg_pc = 0xfe;
        cpu.memory.mem[0xfe] = 0xbe;

        let result = super::zero_page_x(&mut cpu);
        assert_eq!(result.value, 0xbe);
    }

    #[test]
    fn abs() {
        let mut cpu = Cpu::new();

        cpu.reg_pc = 0xfe;
        cpu.memory.mem[0xfe] = 0xef;
        cpu.memory.mem[0xff] = 0xbe;

        let result = super::abs(&mut cpu);

        assert_eq!(result.value, 0xbeef);
    }

    #[test]
    fn abs_x() {
        let mut cpu = Cpu::new();

        cpu.reg_pc = 0xfe;
        cpu.reg_x = 0x01;
        cpu.memory.mem[0xfe] = 0xee;
        cpu.memory.mem[0xff] = 0xbe;

        let result = super::abs_x(&mut cpu);
        assert_eq!(result.value, 0xbeef);
    }

    #[test]
    fn abs_y() {
        let mut cpu = Cpu::new();

        cpu.reg_pc = 0xfe;
        cpu.reg_y = 0x01;
        cpu.memory.mem[0xfe] = 0xee;
        cpu.memory.mem[0xff] = 0xbe;

        let result = super::abs_y(&mut cpu);
        assert_eq!(result.value, 0xbeef);
    }

    #[test]
    fn ind_x() {
        let mut cpu = Cpu::new();

        cpu.reg_pc = 0xfd;
        cpu.reg_x = 0x01;
        cpu.memory.mem[0xfd] = 0xbd;
        cpu.memory.mem[0xbe] = 0xef;

        let result = super::ind_x(&mut cpu);
        assert_eq!(result.value, 0xef);
    }

    #[test]
    fn ind_y() {
        let mut cpu = Cpu::new();

        cpu.reg_pc = 0xfd;

        cpu.reg_y = 0x01;
        cpu.memory.mem[0xfd] = 0xfe;
        cpu.memory.mem[0xfe] = 0xee;
        cpu.memory.mem[0xff] = 0xbe;

        let result = super::ind_y(&mut cpu);

        assert_eq!(result.value, 0xbeef);
    }
}