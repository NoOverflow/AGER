use super::bin_utils::BinUtils;
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
    pub fn ld_nn(reg: &mut u16, v: u16) {
        *reg = v;
    }

    pub fn ld_n(reg: &mut u8, v: u8) {
        *reg = v;
    }

    pub fn xor(a_reg: &mut u8, f_reg: &mut FRegister, v: u8) {
        *a_reg ^= v;
        f_reg.zero = *a_reg == 0;
        f_reg.substract = false;
        f_reg.half_carry = false;
        f_reg.carry = false;
    }

    pub fn dec_nn(high_reg: &mut u8, low_reg: &mut u8) {
        let v: u16 = BinUtils::u16_from_u8s(*high_reg, *low_reg);
        let vs: (u8, u8) = BinUtils::u8s_from_u16(v - 1);

        *high_reg = vs.0;
        *low_reg = vs.1;
    }

    pub fn bit(bit: u8, reg: u8, f_reg: &mut FRegister) {
        f_reg.zero = (reg & (1 << bit)) == 0;
        f_reg.substract = false;
        f_reg.half_carry = true;
    }

    pub fn jr_n(offset: i8, pc: &mut u16) {
        *pc = pc.wrapping_add(offset as u16);
    }
}

// TODO: Move to another file
impl Cpu {
    // Load 16bits value to a 16bits register
}
