use super::InstrResult;
use cpu;
use super::addr;

use std::fmt;

pub fn tax(cpu: &mut cpu::Cpu) -> Box<InstrResult> {
    transfer("tax", TransferLocation::Register(cpu::Register::A), TransferLocation::Register(cpu::Register::X))
}

pub fn tay(cpu: &mut cpu::Cpu) -> Box<InstrResult> {
    transfer("tay", TransferLocation::Register(cpu::Register::A), TransferLocation::Register(cpu::Register::Y))
}

pub fn tsx(cpu: &mut cpu::Cpu) -> Box<InstrResult> {
    transfer("tsx", TransferLocation::Register(cpu::Register::SP), TransferLocation::Register(cpu::Register::X))
}

pub fn txa(cpu: &mut cpu::Cpu) -> Box<InstrResult> {
    transfer("txa", TransferLocation::Register(cpu::Register::X), TransferLocation::Register(cpu::Register::A))
}

pub fn txs(cpu: &mut cpu::Cpu) -> Box<InstrResult> {
    transfer("txs", TransferLocation::Register(cpu::Register::X), TransferLocation::Register(cpu::Register::SP))
}

pub fn tya(cpu: &mut cpu::Cpu) -> Box<InstrResult> {
    transfer("tya", TransferLocation::Register(cpu::Register::Y), TransferLocation::Register(cpu::Register::A))
}

enum TransferLocation {
    Register(cpu::Register),
    Memory(u16),
}

struct TransferInstruction {
    bytes: u8,
    cycles: u8,
    from: TransferLocation,
    to: TransferLocation,
    instr_name: &'static str,
}

fn transfer(instr_name: &'static str, from: TransferLocation, to: TransferLocation) -> Box<InstrResult> {
    Box::new(TransferInstruction {
        bytes: 1,
        cycles: 2,
        from: from,
        to: to,
        instr_name: instr_name,
    })
}

impl InstrResult for TransferInstruction {
    fn run(&self, cpu: &mut cpu::Cpu) {
        let value = match &self.from {
            &TransferLocation::Register(ref register) => {
                match register {
                    &cpu::Register::A => cpu.reg_acc as u8,
                    &cpu::Register::X => cpu.reg_x as u8,
                    &cpu::Register::Y => cpu.reg_y as u8,
                    &cpu::Register::SP => cpu.reg_sp
                }
            }
            &TransferLocation::Memory(ref address) => cpu.memory.read_u8_at(address),
        };

        match &self.to {
            &TransferLocation::Register(ref register) => {
                match register {
                    &cpu::Register::A => cpu.reg_acc = value as i8,
                    &cpu::Register::X => cpu.reg_x = value as i8,
                    &cpu::Register::Y => cpu.reg_y = value as i8,
                    &cpu::Register::SP => cpu.reg_sp = value
                }
            }
            &TransferLocation::Memory(ref address) => cpu.memory.write_at(address, &[value as u8]),
        };
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for TransferInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::debug_fmt(self.instr_name, &addr::implicit()))
    }
}
