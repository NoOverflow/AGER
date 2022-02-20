use super::memory::Memory;
use super::registers::Registers;

pub struct Cpu {
    pub registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
        }
    }

    pub fn cycle(&mut self, mem: &mut Memory) {
        let inst_op_code: u8 = mem.read_u8(self.registers.pc as usize);

        println!("Executing OpCode: {:#02x}", inst_op_code);
    }
}
