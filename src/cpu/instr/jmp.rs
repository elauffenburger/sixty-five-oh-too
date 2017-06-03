use super::Cpu;
use super::super::addr;
use self::addr::AddrResult;
use super::InstrResult;

fn abs(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::abs(cpu);

    jmp(addr_result, 3, 3)
}

fn ind(cpu: &mut Cpu) -> Box<InstrResult> {
    let addr_result = addr::ind(cpu);

    jmp(addr_result, 3, 5)
}

fn jmp(addr_result: AddrResult, bytes: u8, cycles: u8) -> Box<InstrResult> {
    Box::new(JmpInstrResult {
        bytes: bytes,
        cycles: cycles,
        address: addr_result.value
    })    
}

struct JmpInstrResult {
    bytes: u8,
    cycles: u8,
    address: u16
}

impl InstrResult for JmpInstrResult {
   fn run(&self, cpu: &mut Cpu) {
       cpu.reg_pc = self.address
   }
 
   fn get_num_cycles(&self) -> u8 {
       self.cycles
   }
}