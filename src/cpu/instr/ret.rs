use super::InstrResult;
use super::Cpu;
use cpu;

extern crate byteorder;
use self::byteorder::{ ByteOrder };

enum ReturnFrom {
    Interrupt,
    Subroutine
}

#[allow(unused_variables)]
pub fn rti(cpu: &mut Cpu) -> Box<InstrResult> {
    ret(ReturnFrom::Interrupt)
}

#[allow(unused_variables)]
pub fn rts(cpu: &mut Cpu) -> Box<InstrResult> {
    ret(ReturnFrom::Subroutine)
}

#[allow(unused_variables)]
fn ret(from: ReturnFrom) -> Box<InstrResult> {
    Box::new(ReturnInstrResult {
        from: from
    })
}

struct ReturnInstrResult {
    from: ReturnFrom
}

impl InstrResult for ReturnInstrResult {
    fn run(&self, cpu: &mut Cpu) {
        match self.from {
            ReturnFrom::Interrupt => {
                let status = cpu
                    ::ProcessorStatusRegister
                    ::from(cpu.pop_u8().unwrap());
                
                cpu.reg_status = status;
            },
            ReturnFrom::Subroutine => {
                let addr_lo = cpu.pop_u8().unwrap();
                let addr_hi = cpu.pop_u8().unwrap();

                cpu.reg_pc = byteorder::LittleEndian::read_u16(&[addr_lo, addr_hi]);
            }
        }
    } 

    fn get_num_cycles(&self) -> u8 {
       6 
    }
}