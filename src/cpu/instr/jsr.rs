use super::Cpu;
use super::super::addr;
use super::InstrResult;

pub fn jsr(cpu: &mut Cpu) -> Box<InstrResult> {
    let address = addr::abs(cpu).value;

    Box::new(JsrInstrResult {
        bytes: 3,
        cycles: 6,
        address: address
    })    
}

struct JsrInstrResult {
    bytes: u8,
    cycles: u8,
    address: u16
}

impl InstrResult for JsrInstrResult {
   fn run(&self, cpu: &mut Cpu) {
       let rts_addr = cpu.reg_pc - 0x01;

       let pc_hi = ((rts_addr & 0xff00) >> 8) as u8;
       let pc_lo = (rts_addr & 0x00ff) as u8 ;

       cpu.push_u8(pc_hi);
       cpu.push_u8(pc_lo);

       cpu.reg_pc = self.address;
   }
 
   fn get_num_cycles(&self) -> u8 {
       self.cycles
   }
}