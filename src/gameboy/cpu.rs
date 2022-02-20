use super::memory::Memory;
use super::registers::FRegister;
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

    pub fn fetch_u8(&mut self, mem: &mut Memory) -> u8 {
        let v: u8 = mem.read_u8(self.registers.pc as usize);

        self.registers.pc += 1;
        v
    }

    pub fn fetch_u16(&mut self, mem: &mut Memory) -> u16 {
        let low: u16 = self.fetch_u8(mem) as u16;
        let high: u16 = self.fetch_u8(mem) as u16;

        return high << 8 | low;
    }

    fn call_extended(&mut self, mem: &mut Memory, op_code: u8) {}

    fn call(&mut self, mem: &mut Memory, op_code: u8) {
        match op_code {
            0x31 => {
                let v: u16 = self.fetch_u16(mem);

                Instructions::ld(&mut self.registers.sp, &mut self.registers.f, v);
            }
            _ => panic!("{:#02x} is not an implemented opcode.", op_code),
        }
    }

    pub fn cycle(&mut self, mem: &mut Memory) {
        let inst_op_code: u8 = self.fetch_u8(mem);

        println!("Executing OpCode: {:#02x}", inst_op_code);
        self.call(mem, inst_op_code);
    }
}

struct Instructions {}
impl Instructions {
    pub fn ld(reg: &mut u16, flags: &mut FRegister, v: u16) {
        *reg = v;
    }
}

// TODO: Move to another file
impl Cpu {
    // Load 16bits value to a 16bits register
}
