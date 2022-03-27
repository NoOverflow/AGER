use super::bin_utils::BinUtils;
use super::memory::Memory;
use super::registers::FRegister;
use super::registers::Registers;

pub struct Cpu {
    pub registers: Registers,
    pub ime: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
            ime: true,
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

    fn call_extended(&mut self, mem: &mut Memory, op_code: u8) -> u8 {
        match op_code {
            0x11 => {
                Instructions::rl(&mut self.registers.c, &mut self.registers.f);
                8
            }
            0x7C => {
                Instructions::bit(7, self.registers.h, &mut self.registers.f);
                8
            }
            _ => panic!("{:#02x} is not an implemented extended opcode.", op_code),
        }
    }

    fn call(&mut self, mem: &mut Memory, op_code: u8) -> u8 {
        match op_code {
            0x0 => 4,
            0x1 => {
                let dw: u16 = self.fetch_u16(mem);
                let u8s: (u8, u8) = BinUtils::u8s_from_u16(dw);

                self.registers.b = u8s.0;
                self.registers.c = u8s.1;
                12
            }
            0x4 => {
                Instructions::inc(&mut self.registers.b, &mut self.registers.f);
                4
            }
            0x5 => {
                Instructions::dec(&mut self.registers.b, &mut self.registers.f);
                4
            }
            0x6 => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::ld_n(&mut self.registers.b, v);
                8
            }
            0xB => {
                Instructions::dec_nn(&mut self.registers.b, &mut self.registers.c);
                8
            }
            0xC => {
                Instructions::inc(&mut self.registers.c, &mut self.registers.f);
                4
            }
            0xD => {
                Instructions::dec(&mut self.registers.c, &mut self.registers.f);
                4
            }
            0xE => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::ld_n(&mut self.registers.c, v);
                8
            }
            0x11 => {
                let dw: u16 = self.fetch_u16(mem);
                let u8s: (u8, u8) = BinUtils::u8s_from_u16(dw);

                self.registers.d = u8s.0;
                self.registers.e = u8s.1;
                12
            }
            0x13 => {
                Instructions::inc_nn(&mut self.registers.d, &mut self.registers.e);
                8
            }
            0x15 => {
                Instructions::dec(&mut self.registers.d, &mut self.registers.f);
                4
            }
            0x16 => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::ld_n(&mut self.registers.d, v);
                8
            }
            0x17 => {
                Instructions::rl(&mut self.registers.a, &mut self.registers.f);
                4
            }
            0x18 => {
                let offset: i8 = self.fetch_u8(mem) as i8;

                Instructions::jr_n(offset, &mut self.registers.pc);
                12
            }
            0x1A => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.d, self.registers.e);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::ld_n(&mut self.registers.a, v);
                8
            }
            0x1B => {
                Instructions::dec_nn(&mut self.registers.d, &mut self.registers.e);
                8
            }
            0x1D => {
                Instructions::dec(&mut self.registers.e, &mut self.registers.f);
                4
            }
            0x1E => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::ld_n(&mut self.registers.e, v);
                8
            }
            0x20 => {
                let offset: i8 = self.fetch_u8(mem) as i8;

                if !self.registers.f.zero {
                    Instructions::jr_n(offset, &mut self.registers.pc);
                    12
                } else {
                    8
                }
            }
            0x21 => {
                let dw: u16 = self.fetch_u16(mem);
                let u8s: (u8, u8) = BinUtils::u8s_from_u16(dw);

                self.registers.h = u8s.0;
                self.registers.l = u8s.1;
                12
            }
            0x22 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);

                mem.write_u8(self.registers.a, address as usize);
                Instructions::inc_nn(&mut self.registers.h, &mut self.registers.l);
                8
            }
            0x23 => {
                Instructions::inc_nn(&mut self.registers.h, &mut self.registers.l);
                8
            }
            0x24 => {
                Instructions::inc(&mut self.registers.h, &mut self.registers.f);
                4
            }
            0x28 => {
                let offset: i8 = self.fetch_u8(mem) as i8;

                if self.registers.f.zero {
                    Instructions::jr_n(offset, &mut self.registers.pc);
                    12
                } else {
                    8
                }
            }
            0x2A => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                self.registers.a = v;
                Instructions::inc_nn(&mut self.registers.h, &mut self.registers.l);
                8
            }
            0x2B => {
                Instructions::dec_nn(&mut self.registers.h, &mut self.registers.l);
                8
            }
            0x2E => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::ld_n(&mut self.registers.l, v);
                8
            }
            0x2F => {
                self.registers.a = !self.registers.a;
                self.registers.f.substract = true;
                self.registers.f.half_carry = true;
                4
            }
            0x31 => {
                self.registers.sp = self.fetch_u16(mem);
                12
            }
            0x32 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);

                mem.write_u8(self.registers.a, address as usize);
                Instructions::dec_nn(&mut self.registers.h, &mut self.registers.l);
                8
            }
            0x36 => {
                let v: u8 = self.fetch_u8(mem);
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);

                mem.write_u8(v, address as usize);
                12
            }
            0x3B => {
                self.registers.sp = self.registers.sp.wrapping_sub(1);
                8
            }
            0x3D => {
                Instructions::dec(&mut self.registers.a, &mut self.registers.f);
                4
            }
            0x3E => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::ld_n(&mut self.registers.a, v);
                8
            }
            0x4F => {
                Instructions::ld_n(&mut self.registers.c, self.registers.a);
                4
            }
            0x57 => {
                Instructions::ld_n(&mut self.registers.d, self.registers.a);
                4
            }
            0x67 => {
                Instructions::ld_n(&mut self.registers.h, self.registers.a);
                4
            }
            0x77 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);

                mem.write_u8(self.registers.a, address as usize);
                8
            }
            0x78 => {
                Instructions::ld_n(&mut self.registers.a, self.registers.b);
                4
            }
            0x7B => {
                Instructions::ld_n(&mut self.registers.a, self.registers.e);
                4
            }
            0x7C => {
                Instructions::ld_n(&mut self.registers.a, self.registers.h);
                4
            }
            0x7D => {
                Instructions::ld_n(&mut self.registers.a, self.registers.l);
                4
            }
            0x86 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::add(&mut self.registers.a, v, &mut self.registers.f);
                8
            }
            0x90 => {
                Instructions::sub(
                    &mut self.registers.a,
                    self.registers.b,
                    &mut self.registers.f,
                );
                4
            }
            0xA0 => {
                let v: u8 = self.registers.b;

                Instructions::and(&mut self.registers.a, &mut self.registers.f, v);
                4
            }
            0xA1 => {
                let v: u8 = self.registers.c;

                Instructions::and(&mut self.registers.a, &mut self.registers.f, v);
                4
            }
            0xA2 => {
                let v: u8 = self.registers.d;

                Instructions::and(&mut self.registers.a, &mut self.registers.f, v);
                4
            }
            0xA3 => {
                let v: u8 = self.registers.e;

                Instructions::and(&mut self.registers.a, &mut self.registers.f, v);
                4
            }
            0xA4 => {
                let v: u8 = self.registers.h;

                Instructions::and(&mut self.registers.a, &mut self.registers.f, v);
                4
            }
            0xA5 => {
                let v: u8 = self.registers.l;

                Instructions::and(&mut self.registers.a, &mut self.registers.f, v);
                4
            }
            0xA6 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::and(&mut self.registers.a, &mut self.registers.f, v);
                8
            }
            0xA7 => {
                let v: u8 = self.registers.a;

                Instructions::and(&mut self.registers.a, &mut self.registers.f, v);
                4
            }
            0xAF => {
                let v: u8 = self.registers.a;

                Instructions::xor(&mut self.registers.a, &mut self.registers.f, v);
                4
            }
            0xB0 => {
                let v: u8 = self.registers.b;

                Instructions::or(&mut self.registers.a, &mut self.registers.f, v);
                4
            }
            0xB1 => {
                let v: u8 = self.registers.c;

                Instructions::or(&mut self.registers.a, &mut self.registers.f, v);
                4
            }
            0xB2 => {
                let v: u8 = self.registers.d;

                Instructions::or(&mut self.registers.a, &mut self.registers.f, v);
                4
            }
            0xB3 => {
                let v: u8 = self.registers.e;

                Instructions::or(&mut self.registers.a, &mut self.registers.f, v);
                4
            }
            0xB4 => {
                let v: u8 = self.registers.h;

                Instructions::or(&mut self.registers.a, &mut self.registers.f, v);
                4
            }
            0xB5 => {
                let v: u8 = self.registers.l;

                Instructions::or(&mut self.registers.a, &mut self.registers.f, v);
                4
            }
            0xB6 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::or(&mut self.registers.a, &mut self.registers.f, v);
                8
            }
            0xB7 => {
                let v: u8 = self.registers.a;

                Instructions::or(&mut self.registers.a, &mut self.registers.f, v);
                4
            }
            0xBE => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::cp(self.registers.a, v, &mut self.registers.f);
                8
            }
            0xC1 => {
                let v: u16 = self.pop_word(mem);
                let v8s: (u8, u8) = BinUtils::u8s_from_u16(v);

                self.registers.b = v8s.0;
                self.registers.c = v8s.1;
                12
            }
            0xC3 => {
                let address: u16 = self.fetch_u16(mem);

                self.registers.pc = address;
                12
            }
            0xC5 => {
                let word: u16 = BinUtils::u16_from_u8s(self.registers.b, self.registers.c);

                self.push_word(mem, word);
                16
            }
            0xC9 => {
                let address: u16 = self.pop_word(mem);

                self.registers.pc = address;
                16
            }
            0xCB => {
                let extended_op_code: u8 = self.fetch_u8(mem);

                return self.call_extended(mem, extended_op_code);
            }
            0xCD => {
                let dw: u16 = self.fetch_u16(mem);

                self.push_word(mem, self.registers.pc);
                self.registers.pc = dw;
                12
            }
            0xE0 => {
                let dv: u8 = self.fetch_u8(mem);
                let address: u16 = 0xFF00 | dv as u16;

                mem.write_u8(self.registers.a, address as usize);
                12
            }
            0xE2 => {
                mem.write_u8(self.registers.c, 0xFF00 | (self.registers.c as usize));
                8
            }
            0xE6 => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::and(&mut self.registers.a, &mut self.registers.f, v);
                8
            }
            0xEA => {
                let address: u16 = self.fetch_u16(mem);

                mem.write_u8(self.registers.a, address as usize);
                16
            }
            0xF0 => {
                let dv: u8 = self.fetch_u8(mem);
                let address: u16 = 0xFF00 | dv as u16;

                self.registers.a = mem.read_u8(address as usize);
                12
            }
            0xF3 => {
                self.ime = false;
                4
            }
            0xF6 => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::or(&mut self.registers.a, &mut self.registers.f, v);
                8
            }
            0xFB => {
                self.ime = true;
                4
            }
            0xFE => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::cp(self.registers.a, v, &mut self.registers.f);
                8
            }
            _ => panic!(
                "{:#02x} is not an implemented opcode. (PC={:#02x})",
                op_code,
                self.registers.pc - 1
            ),
        }
    }

    pub fn cycle(&mut self, mem: &mut Memory) -> usize {
        let inst_op_code: u8 = self.fetch_u8(mem);

        return self.call(mem, inst_op_code) as usize;
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

    pub fn or(a_reg: &mut u8, f_reg: &mut FRegister, v: u8) {
        *a_reg |= v;
        f_reg.zero = *a_reg == 0;
        f_reg.substract = false;
        f_reg.half_carry = false;
        f_reg.carry = false;
    }

    pub fn and(a_reg: &mut u8, f_reg: &mut FRegister, v: u8) {
        *a_reg &= v;
        f_reg.zero = *a_reg == 0;
        f_reg.substract = false;
        f_reg.half_carry = true;
        f_reg.carry = false;
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
        let vs: (u8, u8) = BinUtils::u8s_from_u16(v.wrapping_sub(1));

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
