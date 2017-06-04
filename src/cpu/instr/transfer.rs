use super::InstrResult;
use cpu;
use super::addr;

use std::fmt;

enum TransferLocation {
    Register(cpu::Register),
    Memory(u16)
}

struct TransferInstruction {
    bytes: u8,
    cycles: u8,
    from: TransferLocation,
    to: TransferLocation,
    instr_name: &'static str
}

fn transfer(instr_name: &'static str, from: TransferLocation, to: TransferLocation) -> Box<InstrResult> {
    Box::new(TransferInstruction {
        bytes: 1,
        cycles: 2,
        from: from,
        to: to,
        instr_name: instr_name
    })
}

impl InstrResult for TransferInstruction {
    fn run(&self, cpu: &mut cpu::Cpu) {
        let value = match &self.from {
            &TransferLocation::Register(ref register) => {
                match register {
                    &cpu::Register::A => cpu.reg_acc,
                    &cpu::Register::X => cpu.reg_x,
                    &cpu::Register::Y => cpu.reg_y,
                }
            },
            &TransferLocation::Memory(ref address) => {
                cpu.memory.read_u8_at(address) as i8
            }
        };

        match &self.to {
            &TransferLocation::Register(ref register) => {
                match register {
                    &cpu::Register::A => cpu.reg_acc = value,
                    &cpu::Register::X => cpu.reg_x = value,
                    &cpu::Register::Y => cpu.reg_y = value,
                }
            },
            &TransferLocation::Memory(ref address) => {
                cpu.memory.write_at(address, &[value as u8])
            }
        };
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for TransferInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::debug_fmt(self.instr_name, addr::implicit()))
    }
}