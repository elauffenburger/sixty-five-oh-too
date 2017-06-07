use super::super::addr;
use super::super::addr::AddrResult;
use super::InstrResult;
use cpu::Cpu;

use std::fmt;

#[allow(unused_variables)]
pub fn acc(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::acc(cpu);

    asl(res, 1, 2, true)
}

pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::zero_page(cpu);

    asl(res, 2, 5, false)
}

pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::zero_page_x(cpu);

    asl(res, 2, 6, false)
}

pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::abs(cpu);

    asl(res, 3, 6, false)
}

pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::abs_x(cpu);

    asl(res, 3, 7, false)
}

fn asl(addr_result: AddrResult, bytes: u8, cycles: u8, is_acc: bool) -> Box<InstrResult> {
    Box::new(AslResult {
        bytes: bytes,
        cycles: cycles,
        is_acc: is_acc,
        addr_result: addr_result,
    })
}

struct AslResult {
    bytes: u8,
    cycles: u8,
    is_acc: bool,
    addr_result: AddrResult,
}

impl InstrResult for AslResult {
    fn run(&self, cpu: &mut Cpu) {
        let address = self.addr_result.value;

        let original_value = self.addr_result.resolve(cpu);

        let new_value = original_value << 0x01;

        cpu.reg_status.zero = new_value == 0;
        cpu.reg_status.carry = ((original_value & 0b1000_0000) >> 7) == 1;
        cpu.reg_status.negative = ((new_value & 0b1000_0000) >> 7) == 1;

        match self.is_acc {
            true => cpu.reg_acc = new_value as i8,
            _ => cpu.memory.write_at(&address, &[new_value]),
        }
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

impl fmt::Debug for AslResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", super::debug_fmt("asl", &self.addr_result))
    }
}

#[cfg(test)]
mod test {
    use cpu::Cpu;

    #[test]
    fn asl() {
        let mut cpu = Cpu::new();
        cpu.reg_pc = 0xfe;

        cpu.memory.mem[0xfe] = 0x01;
        cpu.memory.mem[0x01] = 0b0000_1000;

        let result = super::zero_page(&mut cpu);
        result.run(&mut cpu);

        assert_eq!(cpu.memory.read_u8_at(&0x01), 0b0001_0000);
    }
}
