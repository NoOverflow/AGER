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
            0x11 => {
                Instructions::rl(&mut self.registers.c, &mut self.registers.f);
            }
            0x7C => {
                Instructions::bit(7, self.registers.h, &mut self.registers.f);
            }
            _ => panic!("{:#02x} is not an implemented extended opcode.", op_code),
        }
    }

    fn call(&mut self, mem: &mut Memory, op_code: u8) {
        match op_code {
            0x4 => {
                Instructions::inc(&mut self.registers.b, &mut self.registers.f);
            }
            0x5 => {
                Instructions::dec(&mut self.registers.b, &mut self.registers.f);
            }
            0x6 => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::ld_n(&mut self.registers.b, v);
            }
            0xC => {
                Instructions::inc(&mut self.registers.c, &mut self.registers.f);
            }
            0xD => {
                Instructions::dec(&mut self.registers.c, &mut self.registers.f);
            }
            0xE => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::ld_n(&mut self.registers.c, v);
            }
            0x11 => {
                let dw: u16 = self.fetch_u16(mem);
                let u8s: (u8, u8) = BinUtils::u8s_from_u16(dw);

                self.registers.d = u8s.0;
                self.registers.e = u8s.1;
            }
            0x13 => {
                Instructions::inc_nn(&mut self.registers.d, &mut self.registers.e);
            }
            0x15 => {
                Instructions::dec(&mut self.registers.d, &mut self.registers.f);
            }
            0x16 => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::ld_n(&mut self.registers.d, v);
            }
            0x17 => {
                Instructions::rl(&mut self.registers.a, &mut self.registers.f);
            }
            0x18 => {
                let offset: i8 = self.fetch_u8(mem) as i8;

                Instructions::jr_n(offset, &mut self.registers.pc);
            }
            0x1A => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.d, self.registers.e);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::ld_n(&mut self.registers.a, v);
            }
            0x1D => {
                Instructions::dec(&mut self.registers.e, &mut self.registers.f);
            }
            0x1E => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::ld_n(&mut self.registers.e, v);
            }
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
            0x22 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);

                mem.write_u8(self.registers.a, address as usize);
                Instructions::inc_nn(&mut self.registers.h, &mut self.registers.l);
            }
            0x23 => {
                Instructions::inc_nn(&mut self.registers.h, &mut self.registers.l);
            }
            0x24 => {
                Instructions::inc(&mut self.registers.h, &mut self.registers.f);
            }
            0x28 => {
                let offset: i8 = self.fetch_u8(mem) as i8;

                if self.registers.f.zero {
                    Instructions::jr_n(offset, &mut self.registers.pc);
                }
            }
            0x2E => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::ld_n(&mut self.registers.l, v);
            }
            0x31 => {
                let v: u16 = self.fetch_u16(mem);

                Instructions::ld_nn(&mut self.registers.sp, v);
            }
            0x32 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);

                mem.write_u8(self.registers.a, address as usize);
                Instructions::dec_nn(&mut self.registers.h, &mut self.registers.l);
            }
            0x3D => {
                Instructions::dec(&mut self.registers.a, &mut self.registers.f);
            }
            0x3E => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::ld_n(&mut self.registers.a, v);
            }
            0x4F => {
                Instructions::ld_n(&mut self.registers.c, self.registers.a);
            }
            0x57 => {
                Instructions::ld_n(&mut self.registers.d, self.registers.a);
            }
            0x67 => {
                Instructions::ld_n(&mut self.registers.h, self.registers.a);
            }
            0x77 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);

                mem.write_u8(self.registers.a, address as usize);
            }
            0x78 => {
                Instructions::ld_n(&mut self.registers.a, self.registers.b);
            }
            0x7B => {
                Instructions::ld_n(&mut self.registers.a, self.registers.e);
            }
            0x7C => {
                Instructions::ld_n(&mut self.registers.a, self.registers.h);
            }
            0x7D => {
                Instructions::ld_n(&mut self.registers.a, self.registers.l);
            }
            0x86 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::add(&mut self.registers.a, v, &mut self.registers.f);
            }
            0x90 => {
                Instructions::sub(
                    &mut self.registers.a,
                    self.registers.b,
                    &mut self.registers.f,
                );
            }
            0xAF => {
                let v: u8 = self.registers.a;

                Instructions::xor(&mut self.registers.a, &mut self.registers.f, v);
            }
            0xBE => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::cp(self.registers.a, v, &mut self.registers.f);
            }
            0xC1 => {
                let v: u16 = self.pop_word(mem);
                let v8s: (u8, u8) = BinUtils::u8s_from_u16(v);

                self.registers.b = v8s.0;
                self.registers.c = v8s.1;
            }
            0xC5 => {
                let word: u16 = BinUtils::u16_from_u8s(self.registers.b, self.registers.c);

                self.push_word(mem, word);
            }
            0xC9 => {
                let address: u16 = self.pop_word(mem);

                println!("Ret to {:#02x}", address);
                self.registers.pc = address;
            }
            0xCB => {
                let extended_op_code: u8 = self.fetch_u8(mem);

                self.call_extended(mem, extended_op_code);
            }
            0xCD => {
                let dw: u16 = self.fetch_u16(mem);

                println!(
                    "Call to {:#02x} expecting ret on {:#02x}",
                    dw, self.registers.pc
                );
                self.push_word(mem, self.registers.pc);
                self.registers.pc = dw;
            }
            0xE0 => {
                let dv: u8 = self.fetch_u8(mem);
                let address: u16 = 0xFF00 | dv as u16;

                mem.write_u8(self.registers.a, address as usize);
            }
            0xE2 => {
                mem.write_u8(self.registers.c, 0xFF00 | (self.registers.c as usize));
            }
            0xEA => {
                let address: u16 = self.fetch_u16(mem);

                mem.write_u8(self.registers.a, address as usize);
            }
            0xF0 => {
                let dv: u8 = self.fetch_u8(mem);
                let address: u16 = 0xFF00 | dv as u16;

                self.registers.a = mem.read_u8(address as usize);
            }
            0xFE => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::cp(self.registers.a, v, &mut self.registers.f);
            }
            _ => panic!(
                "{:#02x} is not an implemented opcode. (PC={:#02x})",
                op_code,
                self.registers.pc - 1
            ),
        }
    }

    pub fn cycle(&mut self, mem: &mut Memory) {
        let inst_op_code: u8 = self.fetch_u8(mem);

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

    pub fn dec(reg: &mut u8, f_reg: &mut FRegister) {
        *reg = (*reg).wrapping_sub(1);
        f_reg.zero = *reg == 0;
        f_reg.substract = true;
        f_reg.half_carry = (*reg & 0xF) == 0;
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

    pub fn inc(reg: &mut u8, f_reg: &mut FRegister) {
        *reg = (*reg).wrapping_add(1);
        f_reg.zero = *reg == 0;
        f_reg.substract = false;
        f_reg.half_carry = (*reg & 0xF) == 0xF;
    }

    pub fn inc_nn(high_reg: &mut u8, low_reg: &mut u8) {
        let v: u16 = BinUtils::u16_from_u8s(*high_reg, *low_reg);
        let vs: (u8, u8) = BinUtils::u8s_from_u16(v.wrapping_add(1));

        *high_reg = vs.0;
        *low_reg = vs.1;
    }

    pub fn rlc(reg: &mut u8, f_reg: &mut FRegister) {
        let carry: bool = *reg & 0x80 == 0x80;

        *reg = (*reg << 1) | (if carry { 1 } else { 0 });
        f_reg.zero = *reg == 0;
        f_reg.substract = false;
        f_reg.half_carry = false;
        f_reg.carry = carry;
    }

    pub fn rl(reg: &mut u8, f_reg: &mut FRegister) {
        let carry: bool = *reg & 0x80 == 0x80;

        *reg = (*reg << 1) | (if f_reg.carry { 1 } else { 0 });
        f_reg.zero = *reg == 0;
        f_reg.substract = false;
        f_reg.half_carry = false;
        f_reg.carry = carry;
    }

    pub fn cp(a_reg: u8, v: u8, f_reg: &mut FRegister) {
        f_reg.zero = a_reg == v;
        f_reg.substract = true;
        // TODO: Check
        f_reg.half_carry = (a_reg & 0xF) < (v & 0xF);
        f_reg.carry = a_reg < v;
    }

    // TODO: Check HC and C flags
    pub fn sub(a_reg: &mut u8, v: u8, f_reg: &mut FRegister) {
        let result: u8 = a_reg.wrapping_sub(v);

        f_reg.zero = result == 0;
        f_reg.substract = true;
        // ((*a_reg & 0xF) - (v & 0xF)) & 0x10 != 0 ?
        f_reg.half_carry = (*a_reg & 0xF) < (v & 0xF);
        f_reg.carry = *a_reg < v;
        *a_reg = result;
    }

    // TODO: Check HC and C flags
    pub fn add(a_reg: &mut u8, v: u8, f_reg: &mut FRegister) {
        let result: u8 = a_reg.wrapping_add(v);

        f_reg.zero = result == 0;
        f_reg.substract = false;
        f_reg.half_carry = ((*a_reg & 0xF) + (v & 0xF)) & 0x10 != 0;
        f_reg.carry = *a_reg > 255 - v;
        *a_reg = result;
    }
}

// TODO: Move to another file
impl Cpu {
    // Load 16bits value to a 16bits register
}
