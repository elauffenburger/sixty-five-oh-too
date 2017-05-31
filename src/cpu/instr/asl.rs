use super::super::addr;
use super::super::addr::{ AddrResult };
use super::{ InstrResult };
use cpu::Cpu;

pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::zero_page(cpu);

    asl(cpu, &res, 2, 5, false)
}

pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::zero_page_x(cpu);

    asl(cpu, &res, 2, 6, false)
}

pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::abs(cpu);

    asl(cpu, &res, 3, 6, false)
}

pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let res = addr::abs_x(cpu);

    asl(cpu, &res, 3, 7, false)
}

pub fn acc(cpu: &mut Cpu) -> Box<InstrResult> {
    asl(cpu, &AddrResult::default(), 1, 2, true)
}

fn asl(cpu: &mut Cpu, addr_result: &AddrResult, bytes: u8, cycles: u8, is_acc: bool) -> Box<InstrResult> {
    Box::new(AslResult {
        bytes: bytes,
        cycles: cycles,
        is_acc: is_acc,
        address: addr_result.value
    })
}

struct AslResult {
    bytes: u8,
    cycles: u8,
    is_acc: bool,
    address: u16
}

impl InstrResult for AslResult {
    fn run(&self, cpu: &mut Cpu) {
        let original_value = match self.is_acc {
            true => cpu.reg_acc,
            _ => cpu.memory.read_u8_at(&self.address)
        };

        let new_value = original_value << 0x01;

        cpu.reg_status.zero = new_value == 0;
        cpu.reg_status.carry = ((original_value & 0b1000_0000) >> 7) == 1;
        cpu.reg_status.negative = ((new_value & 0b1000_0000) >> 7) == 1;

        match self.is_acc {
            true => cpu.reg_acc = new_value,
            _ => cpu.memory.write_at(&self.address, &[new_value])
        }
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
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