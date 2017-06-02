use super::Cpu;
use super::InstrResult;
use super::super::addr::AddrResult;
use super::super::addr;
use util;

pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::zero_page(cpu);
    
    bit(cpu, &addr_result, 2, 3)
}

pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::zero_page(cpu);

    bit(cpu, &addr_result, 3, 4)
}

pub fn bit(cpu: &mut Cpu, addr_result: &AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
    let mem_value = cpu.memory.mem[addr_result.value as usize];
    let result = (cpu.reg_acc as u8) & mem_value;

    return Box::new(BitResult {
        result: result,
        bytes: bytes,
        cycles: cycles,
        mem_value: mem_value
    })
}

struct BitResult {
    result: u8,
    bytes: u8,
    cycles: u8,
    mem_value: u8
}

impl InstrResult for BitResult {
    fn run(&self, cpu: &mut Cpu) {
        let zero_flag = self.result == 0;
        let overflow_flag = util::test_bit_set(self.mem_value, 6);
        let negative_flag = util::test_bit_set(self.mem_value, 7);

        cpu.reg_status.zero = zero_flag;
        cpu.reg_status.overflow = overflow_flag;
        cpu.reg_status.negative = negative_flag;
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

#[cfg(test)]
mod test {
    use super::Cpu;

    #[test]
    fn bit() {
        let mut cpu = Cpu::new();
        cpu.reg_pc = 0x01;
        cpu.memory.mem[0x01] = 0xff;
        cpu.memory.mem[0x02] = 0x00;
        cpu.memory.mem[0xff] = 0xff;

        cpu.reg_acc = 0x0f;

        let bit_result = super::abs(&mut cpu);
        bit_result.run(&mut cpu);

        assert_eq!(cpu.reg_status.zero, false);
        assert_eq!(cpu.reg_status.overflow, true);
        assert_eq!(cpu.reg_status.negative, true);
    }
}