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

    pub fn push(&mut self, mem: &mut Memory, v: u8) {
        self.registers.sp -= 1;
        mem.write_u8(v, self.registers.sp as usize);
    }

    pub fn push_word(&mut self, mem: &mut Memory, v: u16) {
        let u8s: (u8, u8) = BinUtils::u8s_from_u16(v);

        // Store LS Byte first
        self.push(mem, u8s.1);
        self.push(mem, u8s.0);
    }

    pub fn pop(&mut self, mem: &mut Memory) -> u8 {
        let v: u8 = mem.read_u8(self.registers.sp as usize);

        self.registers.sp += 1;
        v
    }

    pub fn pop_word(&mut self, mem: &mut Memory) -> u16 {
        let high: u8 = self.pop(mem);
        let low: u8 = self.pop(mem);

        return BinUtils::u16_from_u8s(high, low);
    }
    fn call_extended(&mut self, mem: &mut Memory, op_code: u8) {
        match op_code {
            0x7C => {
                Instructions::bit(7, self.registers.h, &mut self.registers.f);
            }
            _ => panic!("{:#02x} is not an implemented extended opcode.", op_code),
        }
    }

    fn call(&mut self, mem: &mut Memory, op_code: u8) {
        match op_code {
            0x20 => {
                let offset: i8 = self.fetch_u8(mem) as i8;

                if !self.registers.f.zero {
                    Instructions::jr_n(offset, &mut self.registers.pc);
                }
            }
            0x21 => {
                self.registers.l = self.fetch_u8(mem);
                self.registers.h = self.fetch_u8(mem);
            }
            0x31 => {
                let v: u16 = self.fetch_u16(mem);

                Instructions::ld_nn(&mut self.registers.sp, v);
            }
            0x32 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.h);

                mem.write_u8(self.registers.a, address as usize);
                Instructions::dec_nn(&mut self.registers.h, &mut self.registers.l);
            }
            0xAF => {
                let v: u8 = self.registers.a;

                Instructions::xor(&mut self.registers.a, &mut self.registers.f, v);
            }
            0xCB => {
                let extended_op_code: u8 = self.fetch_u8(mem);

                println!("Executing extended OpCode: {:#02x}", extended_op_code);
                self.call_extended(mem, extended_op_code);
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
