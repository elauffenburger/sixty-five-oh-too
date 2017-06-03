use super::Cpu;
use super::InstrResult;
use super::super::addr;
use self::addr::AddrResult;

pub fn imm(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::imm(cpu);

    adc(cpu, addr_result, 2, 2)
}

pub fn zero_page(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::zero_page(cpu);

    adc(cpu, addr_result, 2, 3)
}

pub fn zero_page_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::zero_page_x(cpu);

    adc(cpu, addr_result, 2, 4)
}

pub fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs(cpu);

    adc(cpu, addr_result, 3, 4)
}

pub fn abs_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs_x(cpu);

    adc(cpu, addr_result, 3, 4)
}

pub fn abs_y(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs_y(cpu);

    adc(cpu, addr_result, 3, 4)
}

pub fn ind_x(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::ind_x(cpu);

    adc(cpu, addr_result, 2, 6)
}

pub fn ind_y(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::ind_y(cpu);

    adc(cpu, addr_result, 2, 5)
}

fn adc(cpu: &mut Cpu, addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
    let total_cycles = match addr_result.crosses_boundary.unwrap_or(false) {
        true => cycles + 1,
        false => cycles
    };

    Box::new(AdcInstrResult {
        bytes: bytes,
        cycles: total_cycles,
        to_add: addr_result.value as i8
    })
}

struct AdcInstrResult {
    bytes: u8,
    cycles: u8,
    to_add: i8
}

impl InstrResult for AdcInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        let mut binary_result = (self.to_add as u8 as u16) + (cpu.reg_acc as u8 as u16);

        let (mut result, mut overflowing) = self.to_add.overflowing_add(cpu.reg_acc as i8);
        if cpu.reg_status.carry {
            binary_result += 1;

            let (add_carry_result, add_carry_overflowing) = result.overflowing_add(1);

            result = add_carry_result;
            overflowing = add_carry_overflowing || overflowing;
        }

        cpu.reg_acc = result;
        cpu.reg_status.carry = (binary_result & 0xff00) != 0;
        cpu.reg_status.overflow = overflowing;
        cpu.reg_status.negative = result < 0;
        cpu.reg_status.zero = result == 0;
    }

    fn get_num_cycles(&self) -> u8 {
        self.cycles
    }
}

#[cfg(test)]
mod test {
    use std;
    use super::Cpu;

    fn test_adc(
        cpu: &mut Cpu,
        acc: i8, 
        imm: i8, 
        expected_result: i8, 
        should_carry: bool,
        should_overflow: bool, 
        is_negative: bool
    ) {
        cpu.reg_pc = 0x01;
        cpu.memory.mem[0x01] = imm as u8;
        cpu.reg_acc = acc;

        let adc_res = super::imm(cpu);
        adc_res.run(cpu);

        let carry_value = match cpu.reg_status.carry { true => 1, _ => 0 };
        let full_width_value = acc as i16 + imm as i16 + carry_value as i16;

        assert_eq!(cpu.reg_status.overflow, should_overflow);
        assert_eq!(cpu.reg_acc, expected_result);
        assert_eq!(cpu.reg_status.negative, is_negative);
        assert_eq!(cpu.reg_status.carry, should_carry);
    }

    #[test]
    fn adc_carry() {
        let mut cpu = Cpu::new();

        // carries from 255 to 254
        test_adc(&mut cpu, 0xff, 0xff, 0xfe, true, false, true);
    }

    #[test]
    fn adc_overflow() {
        let mut cpu = Cpu::new();
        
        // overflows from 127 to -1
        test_adc(&mut cpu, 0x01, 0x7f, 0x80, false, true, true);

        // overflows and carries from -1 to 0
        test_adc(&mut cpu, 0x80, 0x80, 0x00, true, true, false);
    }
}