use super::util;

use std::fmt;

#[derive(Clone)]
pub struct ProcessorStatusRegister {
    pub negative: bool,
    pub overflow: bool,

    pub brk: bool,
    pub decimal_mode: bool,
    pub irq_disable: bool,
    pub zero: bool,
    pub carry: bool,
}

impl Into<u8> for ProcessorStatusRegister {
    fn into(self) -> u8 {
        let mut result = 0b0010_0000;

        result = util::set_bit(result, 7, self.negative);
        result = util::set_bit(result, 6, self.overflow);
        result = util::set_bit(result, 4, self.brk);
        result = util::set_bit(result, 3, self.decimal_mode);
        result = util::set_bit(result, 2, self.irq_disable);
        result = util::set_bit(result, 1, self.zero);
        result = util::set_bit(result, 0, self.carry);

        result
    }
}

impl From<u8> for ProcessorStatusRegister {
    fn from(val: u8) -> Self {
        let mut status = ProcessorStatusRegister::default();

        status.carry = util::test_bit_set(val, 0);
        status.zero = util::test_bit_set(val, 1);
        status.irq_disable = util::test_bit_set(val, 2);
        status.decimal_mode = util::test_bit_set(val, 3);
        status.brk = util::test_bit_set(val, 4);
        status.overflow = util::test_bit_set(val, 6);
        status.negative = util::test_bit_set(val, 7);

        status
    }
}

impl Default for ProcessorStatusRegister {
    fn default() -> Self {
        ProcessorStatusRegister {
            negative: false,
            overflow: false,
            brk: true,
            decimal_mode: false,
            irq_disable: false,
            zero: false,
            carry: false,
        }
    }
}

impl fmt::Debug for ProcessorStatusRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let byte: u8 = self.clone().into();

        write!(f, "{:x}", byte)
    }
}
