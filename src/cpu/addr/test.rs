use cpu;
use cpu::Cpu;

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
