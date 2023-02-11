use super::bin_utils::BinUtils;
use super::memory::Memory;
use super::registers::FRegister;
use super::registers::Registers;

pub struct Cpu {
    pub registers: Registers,
    pub ime: bool,
    pub ime_next: bool,
    pub imd_next: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
            ime: true,
            ime_next: false,
            imd_next: false,
        }
    }

    pub fn fetch_u8(&mut self, mem: &mut Memory) -> u8 {
        let v: u8 = mem.read_u8(self.registers.pc as usize);

        self.registers.pc = self.registers.pc.wrapping_add(1);
        v
    }

    pub fn fetch_u16(&mut self, mem: &mut Memory) -> u16 {
        let low: u16 = self.fetch_u8(mem) as u16;
        let high: u16 = self.fetch_u8(mem) as u16;

        high << 8 | low
    }

    pub fn push(&mut self, mem: &mut Memory, v: u8) {
        self.registers.sp -= 1;
        mem.write_u8(v, self.registers.sp as usize);
    }

    pub fn push_word(&mut self, mem: &mut Memory, v: u16) {
        let u8s: (u8, u8) = BinUtils::u8s_from_u16(v);

        // Store HS Byte first
        self.push(mem, u8s.0);
        self.push(mem, u8s.1);
    }

    pub fn pop(&mut self, mem: &mut Memory) -> u8 {
        let v: u8 = mem.read_u8(self.registers.sp as usize);

        self.registers.sp = self.registers.sp.wrapping_add(1);
        v
    }

    pub fn pop_word(&mut self, mem: &mut Memory) -> u16 {
        let low: u8 = self.pop(mem);
        let high: u8 = self.pop(mem);

        BinUtils::u16_from_u8s(high, low)
    }

    fn call_extended(&mut self, mem: &mut Memory, op_code: u8) -> u8 {
        match op_code {
            0x0 => {
                Instructions::rlc(&mut self.registers.b, &mut self.registers.f);
                8
            }
            0x1 => {
                Instructions::rlc(&mut self.registers.c, &mut self.registers.f);
                8
            }
            0x2 => {
                Instructions::rlc(&mut self.registers.d, &mut self.registers.f);
                8
            }
            0x3 => {
                Instructions::rlc(&mut self.registers.e, &mut self.registers.f);
                8
            }
            0x4 => {
                Instructions::rlc(&mut self.registers.h, &mut self.registers.f);
                8
            }
            0x5 => {
                Instructions::rlc(&mut self.registers.l, &mut self.registers.f);
                8
            }
            0x6 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                // TODO: Check
                Instructions::rlc(&mut v, &mut self.registers.f);
                mem.write_u8(v, address as usize);
                16
            }
            0x7 => {
                Instructions::rlc(&mut self.registers.a, &mut self.registers.f);
                8
            }
            0x8 => {
                Instructions::rrc(&mut self.registers.b, &mut self.registers.f, true);
                8
            }
            0x9 => {
                Instructions::rrc(&mut self.registers.c, &mut self.registers.f, true);
                8
            }
            0xA => {
                Instructions::rrc(&mut self.registers.d, &mut self.registers.f, true);
                8
            }
            0xB => {
                Instructions::rrc(&mut self.registers.e, &mut self.registers.f, true);
                8
            }
            0xC => {
                Instructions::rrc(&mut self.registers.h, &mut self.registers.f, true);
                8
            }
            0xD => {
                Instructions::rrc(&mut self.registers.l, &mut self.registers.f, true);
                8
            }
            0xE => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                // TODO: Check
                Instructions::rrc(&mut v, &mut self.registers.f, true);
                mem.write_u8(v, address as usize);
                16
            }
            0xF => {
                Instructions::rrc(&mut self.registers.a, &mut self.registers.f, true);
                8
            }
            0x10 => {
                Instructions::rl(&mut self.registers.b, &mut self.registers.f);
                8
            }
            0x11 => {
                Instructions::rl(&mut self.registers.c, &mut self.registers.f);
                8
            }
            0x12 => {
                Instructions::rl(&mut self.registers.d, &mut self.registers.f);
                8
            }
            0x13 => {
                Instructions::rl(&mut self.registers.e, &mut self.registers.f);
                8
            }
            0x14 => {
                Instructions::rl(&mut self.registers.h, &mut self.registers.f);
                8
            }
            0x15 => {
                Instructions::rl(&mut self.registers.l, &mut self.registers.f);
                8
            }
            0x16 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::rl(&mut v, &mut self.registers.f);
                mem.write_u8(v, address as usize);
                8
            }
            0x17 => {
                Instructions::rl(&mut self.registers.a, &mut self.registers.f);
                8
            }
            0x18 => {
                Instructions::rr(&mut self.registers.b, &mut self.registers.f);
                8
            }
            0x19 => {
                Instructions::rr(&mut self.registers.c, &mut self.registers.f);
                8
            }
            0x1A => {
                Instructions::rr(&mut self.registers.d, &mut self.registers.f);
                8
            }
            0x1B => {
                Instructions::rr(&mut self.registers.e, &mut self.registers.f);
                8
            }
            0x1C => {
                Instructions::rr(&mut self.registers.h, &mut self.registers.f);
                8
            }
            0x1D => {
                Instructions::rr(&mut self.registers.l, &mut self.registers.f);
                8
            }
            0x1E => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::rr(&mut v, &mut self.registers.f);
                mem.write_u8(v, address as usize);
                16
            }
            0x1F => {
                Instructions::rr(&mut self.registers.a, &mut self.registers.f);
                8
            }
            0x20 => {
                Instructions::sla(&mut self.registers.b, &mut self.registers.f);
                8
            }
            0x21 => {
                Instructions::sla(&mut self.registers.c, &mut self.registers.f);
                8
            }
            0x22 => {
                Instructions::sla(&mut self.registers.d, &mut self.registers.f);
                8
            }
            0x23 => {
                Instructions::sla(&mut self.registers.e, &mut self.registers.f);
                8
            }
            0x24 => {
                Instructions::sla(&mut self.registers.h, &mut self.registers.f);
                8
            }
            0x25 => {
                Instructions::sla(&mut self.registers.l, &mut self.registers.f);
                8
            }
            0x26 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::sla(&mut v, &mut self.registers.f);
                mem.write_u8(v, address as usize);
                16
            }
            0x27 => {
                Instructions::sla(&mut self.registers.a, &mut self.registers.f);
                8
            }
            0x28 => {
                Instructions::sra(&mut self.registers.b, &mut self.registers.f);
                8
            }
            0x29 => {
                Instructions::sra(&mut self.registers.c, &mut self.registers.f);
                8
            }
            0x2A => {
                Instructions::sra(&mut self.registers.d, &mut self.registers.f);
                8
            }
            0x2B => {
                Instructions::sra(&mut self.registers.e, &mut self.registers.f);
                8
            }
            0x2C => {
                Instructions::sra(&mut self.registers.h, &mut self.registers.f);
                8
            }
            0x2D => {
                Instructions::sra(&mut self.registers.l, &mut self.registers.f);
                8
            }
            0x2E => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::sra(&mut v, &mut self.registers.f);
                mem.write_u8(v, address as usize);
                16
            }
            0x2F => {
                Instructions::sra(&mut self.registers.a, &mut self.registers.f);
                8
            }
            0x30 => {
                Instructions::swap(&mut self.registers.b, &mut self.registers.f);
                8
            }
            0x31 => {
                Instructions::swap(&mut self.registers.c, &mut self.registers.f);
                8
            }
            0x32 => {
                Instructions::swap(&mut self.registers.d, &mut self.registers.f);
                8
            }
            0x33 => {
                Instructions::swap(&mut self.registers.e, &mut self.registers.f);
                8
            }
            0x34 => {
                Instructions::swap(&mut self.registers.h, &mut self.registers.f);
                8
            }
            0x35 => {
                Instructions::swap(&mut self.registers.l, &mut self.registers.f);
                8
            }
            0x36 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::swap(&mut v, &mut self.registers.f);
                mem.write_u8(v, address as usize);
                16
            }
            0x37 => {
                Instructions::swap(&mut self.registers.a, &mut self.registers.f);
                8
            }
            0x38 => {
                Instructions::srl(&mut self.registers.b, &mut self.registers.f);
                8
            }
            0x39 => {
                Instructions::srl(&mut self.registers.c, &mut self.registers.f);
                8
            }
            0x3A => {
                Instructions::srl(&mut self.registers.d, &mut self.registers.f);
                8
            }
            0x3B => {
                Instructions::srl(&mut self.registers.e, &mut self.registers.f);
                8
            }
            0x3C => {
                Instructions::srl(&mut self.registers.h, &mut self.registers.f);
                8
            }
            0x3D => {
                Instructions::srl(&mut self.registers.l, &mut self.registers.f);
                8
            }
            0x3E => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::srl(&mut v, &mut self.registers.f);
                mem.write_u8(v, address as usize);
                16
            }
            0x3F => {
                Instructions::srl(&mut self.registers.a, &mut self.registers.f);
                8
            }
            0x40 => {
                Instructions::bit(0, self.registers.b, &mut self.registers.f);
                8
            }
            0x41 => {
                Instructions::bit(0, self.registers.c, &mut self.registers.f);
                8
            }
            0x42 => {
                Instructions::bit(0, self.registers.d, &mut self.registers.f);
                8
            }
            0x43 => {
                Instructions::bit(0, self.registers.e, &mut self.registers.f);
                8
            }
            0x44 => {
                Instructions::bit(0, self.registers.h, &mut self.registers.f);
                8
            }
            0x45 => {
                Instructions::bit(0, self.registers.l, &mut self.registers.f);
                8
            }
            0x46 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::bit(0, v, &mut self.registers.f);
                16
            }
            0x47 => {
                Instructions::bit(0, self.registers.a, &mut self.registers.f);
                8
            }
            0x48 => {
                Instructions::bit(1, self.registers.b, &mut self.registers.f);
                8
            }
            0x49 => {
                Instructions::bit(1, self.registers.c, &mut self.registers.f);
                8
            }
            0x4A => {
                Instructions::bit(1, self.registers.d, &mut self.registers.f);
                8
            }
            0x4B => {
                Instructions::bit(1, self.registers.e, &mut self.registers.f);
                8
            }
            0x4C => {
                Instructions::bit(1, self.registers.h, &mut self.registers.f);
                8
            }
            0x4D => {
                Instructions::bit(1, self.registers.l, &mut self.registers.f);
                8
            }
            0x4E => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);
                Instructions::bit(1, v, &mut self.registers.f);
                16
            }
            0x4F => {
                Instructions::bit(1, self.registers.a, &mut self.registers.f);
                8
            }
            0x50 => {
                Instructions::bit(2, self.registers.b, &mut self.registers.f);
                8
            }
            0x51 => {
                Instructions::bit(2, self.registers.c, &mut self.registers.f);
                8
            }
            0x52 => {
                Instructions::bit(2, self.registers.d, &mut self.registers.f);
                8
            }
            0x53 => {
                Instructions::bit(2, self.registers.e, &mut self.registers.f);
                8
            }
            0x54 => {
                Instructions::bit(2, self.registers.h, &mut self.registers.f);
                8
            }
            0x55 => {
                Instructions::bit(2, self.registers.l, &mut self.registers.f);
                8
            }
            0x56 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);
                Instructions::bit(2, v, &mut self.registers.f);
                16
            }
            0x57 => {
                Instructions::bit(2, self.registers.a, &mut self.registers.f);
                8
            }
            0x58 => {
                Instructions::bit(3, self.registers.b, &mut self.registers.f);
                8
            }
            0x59 => {
                Instructions::bit(3, self.registers.c, &mut self.registers.f);
                8
            }
            0x5A => {
                Instructions::bit(3, self.registers.d, &mut self.registers.f);
                8
            }
            0x5B => {
                Instructions::bit(3, self.registers.e, &mut self.registers.f);
                8
            }
            0x5C => {
                Instructions::bit(3, self.registers.h, &mut self.registers.f);
                8
            }
            0x5D => {
                Instructions::bit(3, self.registers.l, &mut self.registers.f);
                8
            }
            0x5E => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);
                Instructions::bit(3, v, &mut self.registers.f);
                16
            }
            0x5F => {
                Instructions::bit(3, self.registers.a, &mut self.registers.f);
                8
            }
            0x60 => {
                Instructions::bit(4, self.registers.b, &mut self.registers.f);
                8
            }
            0x61 => {
                Instructions::bit(4, self.registers.c, &mut self.registers.f);
                8
            }
            0x62 => {
                Instructions::bit(4, self.registers.d, &mut self.registers.f);
                8
            }
            0x63 => {
                Instructions::bit(4, self.registers.e, &mut self.registers.f);
                8
            }
            0x64 => {
                Instructions::bit(4, self.registers.h, &mut self.registers.f);
                8
            }
            0x65 => {
                Instructions::bit(4, self.registers.l, &mut self.registers.f);
                8
            }
            0x66 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);
                Instructions::bit(4, v, &mut self.registers.f);
                16
            }
            0x67 => {
                Instructions::bit(4, self.registers.a, &mut self.registers.f);
                8
            }
            0x68 => {
                Instructions::bit(5, self.registers.b, &mut self.registers.f);
                8
            }
            0x69 => {
                Instructions::bit(5, self.registers.c, &mut self.registers.f);
                8
            }
            0x6A => {
                Instructions::bit(5, self.registers.d, &mut self.registers.f);
                8
            }
            0x6B => {
                Instructions::bit(5, self.registers.e, &mut self.registers.f);
                8
            }
            0x6C => {
                Instructions::bit(5, self.registers.h, &mut self.registers.f);
                8
            }
            0x6D => {
                Instructions::bit(5, self.registers.l, &mut self.registers.f);
                8
            }
            0x6E => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);
                Instructions::bit(5, v, &mut self.registers.f);
                16
            }
            0x6F => {
                Instructions::bit(5, self.registers.a, &mut self.registers.f);
                8
            }
            0x70 => {
                Instructions::bit(6, self.registers.b, &mut self.registers.f);
                8
            }
            0x71 => {
                Instructions::bit(6, self.registers.c, &mut self.registers.f);
                8
            }
            0x72 => {
                Instructions::bit(6, self.registers.d, &mut self.registers.f);
                8
            }
            0x73 => {
                Instructions::bit(6, self.registers.e, &mut self.registers.f);
                8
            }
            0x74 => {
                Instructions::bit(6, self.registers.h, &mut self.registers.f);
                8
            }
            0x75 => {
                Instructions::bit(6, self.registers.l, &mut self.registers.f);
                8
            }
            0x76 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);
                Instructions::bit(6, v, &mut self.registers.f);
                16
            }
            0x77 => {
                Instructions::bit(6, self.registers.a, &mut self.registers.f);
                8
            }
            0x78 => {
                Instructions::bit(7, self.registers.b, &mut self.registers.f);
                8
            }
            0x79 => {
                Instructions::bit(7, self.registers.c, &mut self.registers.f);
                8
            }
            0x7A => {
                Instructions::bit(7, self.registers.d, &mut self.registers.f);
                8
            }
            0x7B => {
                Instructions::bit(7, self.registers.e, &mut self.registers.f);
                8
            }
            0x7C => {
                Instructions::bit(7, self.registers.h, &mut self.registers.f);
                8
            }
            0x7D => {
                Instructions::bit(7, self.registers.l, &mut self.registers.f);
                8
            }
            0x7E => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);
                Instructions::bit(7, v, &mut self.registers.f);
                16
            }
            0x7F => {
                Instructions::bit(7, self.registers.a, &mut self.registers.f);
                8
            }
            0x80 => {
                Instructions::res(&mut self.registers.b, 0);
                8
            }
            0x81 => {
                Instructions::res(&mut self.registers.c, 0);
                8
            }
            0x82 => {
                Instructions::res(&mut self.registers.d, 0);
                8
            }
            0x83 => {
                Instructions::res(&mut self.registers.e, 0);
                8
            }
            0x84 => {
                Instructions::res(&mut self.registers.h, 0);
                8
            }
            0x85 => {
                Instructions::res(&mut self.registers.l, 0);
                8
            }
            0x86 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::res(&mut v, 0);
                mem.write_u8(v, address as usize);
                16
            }
            0x87 => {
                Instructions::res(&mut self.registers.a, 0);
                8
            }
            0x88 => {
                Instructions::res(&mut self.registers.b, 1);
                8
            }
            0x89 => {
                Instructions::res(&mut self.registers.c, 1);
                8
            }
            0x8A => {
                Instructions::res(&mut self.registers.d, 1);
                8
            }
            0x8B => {
                Instructions::res(&mut self.registers.e, 1);
                8
            }
            0x8C => {
                Instructions::res(&mut self.registers.h, 1);
                8
            }
            0x8D => {
                Instructions::res(&mut self.registers.l, 1);
                8
            }
            0x8E => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::res(&mut v, 1);
                mem.write_u8(v, address as usize);
                16
            }
            0x8F => {
                Instructions::res(&mut self.registers.a, 1);
                8
            }
            0x90 => {
                Instructions::res(&mut self.registers.b, 2);
                8
            }
            0x91 => {
                Instructions::res(&mut self.registers.c, 2);
                8
            }
            0x92 => {
                Instructions::res(&mut self.registers.d, 2);
                8
            }
            0x93 => {
                Instructions::res(&mut self.registers.e, 2);
                8
            }
            0x94 => {
                Instructions::res(&mut self.registers.h, 2);
                8
            }
            0x95 => {
                Instructions::res(&mut self.registers.l, 2);
                8
            }
            0x96 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::res(&mut v, 2);
                mem.write_u8(v, address as usize);
                16
            }
            0x97 => {
                Instructions::res(&mut self.registers.a, 2);
                8
            }
            0x98 => {
                Instructions::res(&mut self.registers.b, 3);
                8
            }
            0x99 => {
                Instructions::res(&mut self.registers.c, 3);
                8
            }
            0x9A => {
                Instructions::res(&mut self.registers.d, 3);
                8
            }
            0x9B => {
                Instructions::res(&mut self.registers.e, 3);
                8
            }
            0x9C => {
                Instructions::res(&mut self.registers.h, 3);
                8
            }
            0x9D => {
                Instructions::res(&mut self.registers.l, 3);
                8
            }
            0x9E => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::res(&mut v, 3);
                mem.write_u8(v, address as usize);
                16
            }
            0x9F => {
                Instructions::res(&mut self.registers.a, 3);
                8
            }
            0xA0 => {
                Instructions::res(&mut self.registers.b, 4);
                8
            }
            0xA1 => {
                Instructions::res(&mut self.registers.c, 4);
                8
            }
            0xA2 => {
                Instructions::res(&mut self.registers.d, 4);
                8
            }
            0xA3 => {
                Instructions::res(&mut self.registers.e, 4);
                8
            }
            0xA4 => {
                Instructions::res(&mut self.registers.h, 4);
                8
            }
            0xA5 => {
                Instructions::res(&mut self.registers.l, 4);
                8
            }
            0xA6 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::res(&mut v, 4);
                mem.write_u8(v, address as usize);
                16
            }
            0xA7 => {
                Instructions::res(&mut self.registers.a, 4);
                8
            }
            0xA8 => {
                Instructions::res(&mut self.registers.b, 5);
                8
            }
            0xA9 => {
                Instructions::res(&mut self.registers.c, 5);
                8
            }
            0xAA => {
                Instructions::res(&mut self.registers.d, 5);
                8
            }
            0xAB => {
                Instructions::res(&mut self.registers.e, 5);
                8
            }
            0xAC => {
                Instructions::res(&mut self.registers.h, 5);
                8
            }
            0xAD => {
                Instructions::res(&mut self.registers.l, 5);
                8
            }
            0xAE => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::res(&mut v, 5);
                mem.write_u8(v, address as usize);
                16
            }
            0xAF => {
                Instructions::res(&mut self.registers.a, 5);
                8
            }
            0xB0 => {
                Instructions::res(&mut self.registers.b, 6);
                8
            }
            0xB1 => {
                Instructions::res(&mut self.registers.c, 6);
                8
            }
            0xB2 => {
                Instructions::res(&mut self.registers.d, 6);
                8
            }
            0xB3 => {
                Instructions::res(&mut self.registers.e, 6);
                8
            }
            0xB4 => {
                Instructions::res(&mut self.registers.h, 6);
                8
            }
            0xB5 => {
                Instructions::res(&mut self.registers.l, 6);
                8
            }
            0xB6 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::res(&mut v, 6);
                mem.write_u8(v, address as usize);
                16
            }
            0xB7 => {
                Instructions::res(&mut self.registers.a, 6);
                8
            }
            0xB8 => {
                Instructions::res(&mut self.registers.b, 7);
                8
            }
            0xB9 => {
                Instructions::res(&mut self.registers.c, 7);
                8
            }
            0xBA => {
                Instructions::res(&mut self.registers.d, 7);
                8
            }
            0xBB => {
                Instructions::res(&mut self.registers.e, 7);
                8
            }
            0xBC => {
                Instructions::res(&mut self.registers.h, 7);
                8
            }
            0xBD => {
                Instructions::res(&mut self.registers.l, 7);
                8
            }
            0xBE => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::res(&mut v, 7);
                mem.write_u8(v, address as usize);
                16
            }
            0xBF => {
                Instructions::res(&mut self.registers.a, 7);
                8
            }
            0xC0 => {
                Instructions::set(0, &mut self.registers.b);
                8
            }
            0xC1 => {
                Instructions::set(0, &mut self.registers.c);
                8
            }
            0xC2 => {
                Instructions::set(0, &mut self.registers.d);
                8
            }
            0xC3 => {
                Instructions::set(0, &mut self.registers.e);
                8
            }
            0xC4 => {
                Instructions::set(0, &mut self.registers.h);
                8
            }
            0xC5 => {
                Instructions::set(0, &mut self.registers.l);
                8
            }
            0xC6 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::set(0, &mut v);
                mem.write_u8(v, address as usize);
                16
            }
            0xC7 => {
                Instructions::set(0, &mut self.registers.a);
                8
            }
            0xC8 => {
                Instructions::set(1, &mut self.registers.b);
                8
            }
            0xC9 => {
                Instructions::set(1, &mut self.registers.c);
                8
            }
            0xCA => {
                Instructions::set(1, &mut self.registers.d);
                8
            }
            0xCB => {
                Instructions::set(1, &mut self.registers.e);
                8
            }
            0xCC => {
                Instructions::set(1, &mut self.registers.h);
                8
            }
            0xCD => {
                Instructions::set(1, &mut self.registers.l);
                8
            }
            0xCE => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::set(1, &mut v);
                mem.write_u8(v, address as usize);
                16
            }
            0xCF => {
                Instructions::set(1, &mut self.registers.a);
                8
            }
            0xD0 => {
                Instructions::set(2, &mut self.registers.b);
                8
            }
            0xD1 => {
                Instructions::set(2, &mut self.registers.c);
                8
            }
            0xD2 => {
                Instructions::set(2, &mut self.registers.d);
                8
            }
            0xD3 => {
                Instructions::set(2, &mut self.registers.e);
                8
            }
            0xD4 => {
                Instructions::set(2, &mut self.registers.h);
                8
            }
            0xD5 => {
                Instructions::set(2, &mut self.registers.l);
                8
            }
            0xD6 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::set(2, &mut v);
                mem.write_u8(v, address as usize);
                16
            }
            0xD7 => {
                Instructions::set(2, &mut self.registers.a);
                8
            }
            0xD8 => {
                Instructions::set(3, &mut self.registers.b);
                8
            }
            0xD9 => {
                Instructions::set(3, &mut self.registers.c);
                8
            }
            0xDA => {
                Instructions::set(3, &mut self.registers.d);
                8
            }
            0xDB => {
                Instructions::set(3, &mut self.registers.e);
                8
            }
            0xDC => {
                Instructions::set(3, &mut self.registers.h);
                8
            }
            0xDD => {
                Instructions::set(3, &mut self.registers.l);
                8
            }
            0xDE => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::set(3, &mut v);
                mem.write_u8(v, address as usize);
                16
            }
            0xDF => {
                Instructions::set(3, &mut self.registers.a);
                8
            }
            0xE0 => {
                Instructions::set(4, &mut self.registers.b);
                8
            }
            0xE1 => {
                Instructions::set(4, &mut self.registers.c);
                8
            }
            0xE2 => {
                Instructions::set(4, &mut self.registers.d);
                8
            }
            0xE3 => {
                Instructions::set(4, &mut self.registers.e);
                8
            }
            0xE4 => {
                Instructions::set(4, &mut self.registers.h);
                8
            }
            0xE5 => {
                Instructions::set(4, &mut self.registers.l);
                8
            }
            0xE6 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::set(4, &mut v);
                mem.write_u8(v, address as usize);
                16
            }
            0xE7 => {
                Instructions::set(4, &mut self.registers.a);
                8
            }
            0xE8 => {
                Instructions::set(5, &mut self.registers.b);
                8
            }
            0xE9 => {
                Instructions::set(5, &mut self.registers.c);
                8
            }
            0xEA => {
                Instructions::set(5, &mut self.registers.d);
                8
            }
            0xEB => {
                Instructions::set(5, &mut self.registers.e);
                8
            }
            0xEC => {
                Instructions::set(5, &mut self.registers.h);
                8
            }
            0xED => {
                Instructions::set(5, &mut self.registers.l);
                8
            }
            0xEE => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::set(5, &mut v);
                mem.write_u8(v, address as usize);
                16
            }
            0xEF => {
                Instructions::set(5, &mut self.registers.a);
                8
            }
            0xF0 => {
                Instructions::set(6, &mut self.registers.b);
                8
            }
            0xF1 => {
                Instructions::set(6, &mut self.registers.c);
                8
            }
            0xF2 => {
                Instructions::set(6, &mut self.registers.d);
                8
            }
            0xF3 => {
                Instructions::set(6, &mut self.registers.e);
                8
            }
            0xF4 => {
                Instructions::set(6, &mut self.registers.h);
                8
            }
            0xF5 => {
                Instructions::set(6, &mut self.registers.l);
                8
            }
            0xF6 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::set(6, &mut v);
                mem.write_u8(v, address as usize);
                16
            }
            0xF7 => {
                Instructions::set(6, &mut self.registers.a);
                8
            }
            0xF8 => {
                Instructions::set(7, &mut self.registers.b);
                8
            }
            0xF9 => {
                Instructions::set(7, &mut self.registers.c);
                8
            }
            0xFA => {
                Instructions::set(7, &mut self.registers.d);
                8
            }
            0xFB => {
                Instructions::set(7, &mut self.registers.e);
                8
            }
            0xFC => {
                Instructions::set(7, &mut self.registers.h);
                8
            }
            0xFD => {
                Instructions::set(7, &mut self.registers.l);
                8
            }
            0xFE => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::set(7, &mut v);
                mem.write_u8(v, address as usize);
                16
            }
            0xFF => {
                Instructions::set(7, &mut self.registers.a);
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
            0x2 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.b, self.registers.c);

                mem.write_u8(self.registers.a, address as usize);
                8
            }
            0x3 => {
                Instructions::inc_nn(&mut self.registers.b, &mut self.registers.c);
                8
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
            0x7 => {
                Instructions::rlc(&mut self.registers.a, &mut self.registers.f);
                self.registers.f.zero = false;
                4
            }
            0x8 => {
                let address: u16 = self.fetch_u16(mem);
                let v8s: (u8, u8) = BinUtils::u8s_from_u16(self.registers.sp);

                mem.write_u8(v8s.1, address as usize);
                mem.write_u8(v8s.0, address as usize + 1);
                20
            }
            0x9 => {
                let v: u16 = BinUtils::u16_from_u8s(self.registers.b, self.registers.c);

                Instructions::add_nn(
                    &mut self.registers.h,
                    &mut self.registers.l,
                    v,
                    &mut self.registers.f,
                );
                8
            }
            0xA => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.b, self.registers.c);
                let v: u8 = mem.read_u8(address as usize);

                self.registers.a = v;
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
            0xF => {
                Instructions::rrc(&mut self.registers.a, &mut self.registers.f, true);
                self.registers.f.zero = false;
                4
            }
            0x10 => {
                mem.stopped = true;
                mem.div = 0x0;
                println!("CPU Stopped!");
                4
            }
            0x11 => {
                let dw: u16 = self.fetch_u16(mem);
                let u8s: (u8, u8) = BinUtils::u8s_from_u16(dw);

                self.registers.d = u8s.0;
                self.registers.e = u8s.1;
                12
            }
            0x12 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.d, self.registers.e);

                mem.write_u8(self.registers.a, address as usize);
                8
            }
            0x13 => {
                Instructions::inc_nn(&mut self.registers.d, &mut self.registers.e);
                8
            }
            0x14 => {
                Instructions::inc(&mut self.registers.d, &mut self.registers.f);
                4
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
                self.registers.f.zero = false;
                4
            }
            0x18 => {
                let offset: i8 = self.fetch_u8(mem) as i8;

                Instructions::jr_n(offset, &mut self.registers.pc);
                12
            }
            0x19 => {
                let v: u16 = BinUtils::u16_from_u8s(self.registers.d, self.registers.e);

                Instructions::add_nn(
                    &mut self.registers.h,
                    &mut self.registers.l,
                    v,
                    &mut self.registers.f,
                );
                8
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
            0x1C => {
                Instructions::inc(&mut self.registers.e, &mut self.registers.f);
                4
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
            0x1F => {
                Instructions::rr(&mut self.registers.a, &mut self.registers.f);
                self.registers.f.zero = false;
                4
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
            0x25 => {
                Instructions::dec(&mut self.registers.h, &mut self.registers.f);
                4
            }
            0x26 => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::ld_n(&mut self.registers.h, v);
                8
            }
            0x27 => {
                let mut a: u8 = self.registers.a;
                let mut adjust = if self.registers.f.carry { 0x60 } else { 0x0 };

                if self.registers.f.half_carry {
                    adjust |= 0x06;
                };
                if !self.registers.f.substract {
                    if a & 0x0F > 0x09 {
                        adjust |= 0x06
                    };
                    if a > 0x99 {
                        adjust |= 0x60;
                    };
                    a = a.wrapping_add(adjust);
                } else {
                    a = a.wrapping_sub(adjust);
                }
                self.registers.f.carry = adjust >= 0x60;
                self.registers.f.half_carry = false;
                self.registers.f.zero = a == 0;
                self.registers.a = a;
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
            0x29 => {
                let v: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);

                Instructions::add_nn(
                    &mut self.registers.h,
                    &mut self.registers.l,
                    v,
                    &mut self.registers.f,
                );
                8
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
            0x2C => {
                Instructions::inc(&mut self.registers.l, &mut self.registers.f);
                4
            }
            0x2D => {
                Instructions::dec(&mut self.registers.l, &mut self.registers.f);
                4
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
            0x30 => {
                let offset: i8 = self.fetch_u8(mem) as i8;

                if !self.registers.f.carry {
                    Instructions::jr_n(offset, &mut self.registers.pc);
                    12
                } else {
                    8
                }
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
            0x33 => {
                self.registers.sp = self.registers.sp.wrapping_add(1);
                8
            }
            0x34 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::inc(&mut v, &mut self.registers.f);
                mem.write_u8(v, address as usize);
                12
            }
            0x35 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::dec(&mut v, &mut self.registers.f);
                mem.write_u8(v, address as usize);
                12
            }
            0x36 => {
                let v: u8 = self.fetch_u8(mem);
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);

                mem.write_u8(v, address as usize);
                12
            }
            0x37 => {
                self.registers.f.substract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = true;
                4
            }
            0x38 => {
                let offset: i8 = self.fetch_u8(mem) as i8;

                if self.registers.f.carry {
                    Instructions::jr_n(offset, &mut self.registers.pc);
                    12
                } else {
                    8
                }
            }
            0x39 => {
                Instructions::add_nn(
                    &mut self.registers.h,
                    &mut self.registers.l,
                    self.registers.sp,
                    &mut self.registers.f,
                );
                8
            }
            0x3A => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                self.registers.a = v;
                Instructions::dec_nn(&mut self.registers.h, &mut self.registers.l);
                8
            }
            0x3B => {
                self.registers.sp = self.registers.sp.wrapping_sub(1);
                8
            }
            0x3C => {
                Instructions::inc(&mut self.registers.a, &mut self.registers.f);
                4
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
            0x3F => {
                self.registers.f.carry = !self.registers.f.carry;
                self.registers.f.substract = false;
                self.registers.f.half_carry = false;
                4
            }
            0x40 => {
                let v: u8 = self.registers.b;

                Instructions::ld_n(&mut self.registers.b, v);
                4
            }
            0x41 => {
                Instructions::ld_n(&mut self.registers.b, self.registers.c);
                4
            }
            0x42 => {
                Instructions::ld_n(&mut self.registers.b, self.registers.d);
                4
            }
            0x43 => {
                Instructions::ld_n(&mut self.registers.b, self.registers.e);
                4
            }
            0x44 => {
                Instructions::ld_n(&mut self.registers.b, self.registers.h);
                4
            }
            0x45 => {
                Instructions::ld_n(&mut self.registers.b, self.registers.l);
                4
            }
            0x46 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::ld_n(&mut self.registers.b, v);
                8
            }
            0x47 => {
                Instructions::ld_n(&mut self.registers.b, self.registers.a);
                4
            }
            0x48 => {
                Instructions::ld_n(&mut self.registers.c, self.registers.b);
                4
            }
            0x49 => {
                let v: u8 = self.registers.c;

                Instructions::ld_n(&mut self.registers.c, v);
                4
            }
            0x4A => {
                Instructions::ld_n(&mut self.registers.c, self.registers.d);
                4
            }
            0x4B => {
                Instructions::ld_n(&mut self.registers.c, self.registers.e);
                4
            }
            0x4C => {
                Instructions::ld_n(&mut self.registers.c, self.registers.h);
                4
            }
            0x4D => {
                Instructions::ld_n(&mut self.registers.c, self.registers.l);
                4
            }
            0x4E => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::ld_n(&mut self.registers.c, v);
                8
            }
            0x4F => {
                Instructions::ld_n(&mut self.registers.c, self.registers.a);
                4
            }
            0x50 => {
                Instructions::ld_n(&mut self.registers.d, self.registers.b);
                4
            }
            0x51 => {
                Instructions::ld_n(&mut self.registers.d, self.registers.c);
                4
            }
            0x52 => {
                let v: u8 = self.registers.d;

                Instructions::ld_n(&mut self.registers.d, v);
                4
            }
            0x53 => {
                Instructions::ld_n(&mut self.registers.d, self.registers.e);
                4
            }
            0x54 => {
                Instructions::ld_n(&mut self.registers.d, self.registers.h);
                4
            }
            0x55 => {
                Instructions::ld_n(&mut self.registers.d, self.registers.l);
                4
            }
            0x56 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::ld_n(&mut self.registers.d, v);
                8
            }
            0x57 => {
                Instructions::ld_n(&mut self.registers.d, self.registers.a);
                4
            }
            0x58 => {
                Instructions::ld_n(&mut self.registers.e, self.registers.b);
                4
            }
            0x59 => {
                Instructions::ld_n(&mut self.registers.e, self.registers.c);
                4
            }
            0x5A => {
                Instructions::ld_n(&mut self.registers.e, self.registers.d);
                4
            }
            0x5B => {
                let v: u8 = self.registers.e;

                Instructions::ld_n(&mut self.registers.e, v);
                4
            }
            0x5C => {
                Instructions::ld_n(&mut self.registers.e, self.registers.h);
                4
            }
            0x5D => {
                Instructions::ld_n(&mut self.registers.e, self.registers.l);
                4
            }
            0x5E => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::ld_n(&mut self.registers.e, v);
                8
            }
            0x5F => {
                Instructions::ld_n(&mut self.registers.e, self.registers.a);
                4
            }
            0x60 => {
                Instructions::ld_n(&mut self.registers.h, self.registers.b);
                4
            }
            0x61 => {
                Instructions::ld_n(&mut self.registers.h, self.registers.c);
                4
            }
            0x62 => {
                Instructions::ld_n(&mut self.registers.h, self.registers.d);
                4
            }
            0x63 => {
                Instructions::ld_n(&mut self.registers.h, self.registers.e);
                4
            }
            0x64 => {
                let v: u8 = self.registers.h;

                Instructions::ld_n(&mut self.registers.h, v);
                4
            }
            0x65 => {
                Instructions::ld_n(&mut self.registers.h, self.registers.l);
                4
            }
            0x66 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::ld_n(&mut self.registers.h, v);
                8
            }
            0x67 => {
                Instructions::ld_n(&mut self.registers.h, self.registers.a);
                4
            }
            0x68 => {
                Instructions::ld_n(&mut self.registers.l, self.registers.b);
                4
            }
            0x69 => {
                Instructions::ld_n(&mut self.registers.l, self.registers.c);
                4
            }
            0x6A => {
                Instructions::ld_n(&mut self.registers.l, self.registers.d);
                4
            }
            0x6B => {
                Instructions::ld_n(&mut self.registers.l, self.registers.e);
                4
            }
            0x6C => {
                Instructions::ld_n(&mut self.registers.l, self.registers.h);
                4
            }
            0x6D => {
                let v: u8 = self.registers.l;

                Instructions::ld_n(&mut self.registers.l, v);
                4
            }
            0x6E => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::ld_n(&mut self.registers.l, v);
                8
            }
            0x6F => {
                Instructions::ld_n(&mut self.registers.l, self.registers.a);
                4
            }
            0x70 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);

                mem.write_u8(self.registers.b, address as usize);
                8
            }
            0x71 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);

                mem.write_u8(self.registers.c, address as usize);
                8
            }
            0x72 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);

                mem.write_u8(self.registers.d, address as usize);
                8
            }
            0x73 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);

                mem.write_u8(self.registers.e, address as usize);
                8
            }
            0x74 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);

                mem.write_u8(self.registers.h, address as usize);
                8
            }
            0x75 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);

                mem.write_u8(self.registers.l, address as usize);
                8
            }
            0x76 => {
                mem.halted = true;
                println!("CPU Halted!");
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
            0x79 => {
                Instructions::ld_n(&mut self.registers.a, self.registers.c);
                4
            }
            0x7A => {
                Instructions::ld_n(&mut self.registers.a, self.registers.d);
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
            0x7E => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::ld_n(&mut self.registers.a, v);
                8
            }
            0x7F => {
                let v = self.registers.a;

                Instructions::ld_n(&mut self.registers.a, v);
                4
            }
            0x80 => {
                Instructions::add(
                    &mut self.registers.a,
                    self.registers.b,
                    &mut self.registers.f,
                );
                4
            }
            0x81 => {
                Instructions::add(
                    &mut self.registers.a,
                    self.registers.c,
                    &mut self.registers.f,
                );
                4
            }
            0x82 => {
                Instructions::add(
                    &mut self.registers.a,
                    self.registers.d,
                    &mut self.registers.f,
                );
                4
            }
            0x83 => {
                Instructions::add(
                    &mut self.registers.a,
                    self.registers.e,
                    &mut self.registers.f,
                );
                4
            }
            0x84 => {
                Instructions::add(
                    &mut self.registers.a,
                    self.registers.h,
                    &mut self.registers.f,
                );
                4
            }
            0x85 => {
                Instructions::add(
                    &mut self.registers.a,
                    self.registers.l,
                    &mut self.registers.f,
                );
                4
            }
            0x86 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::add(&mut self.registers.a, v, &mut self.registers.f);
                8
            }
            0x87 => {
                let v: u8 = self.registers.a;

                Instructions::add(&mut self.registers.a, v, &mut self.registers.f);
                4
            }
            0x88 => {
                Instructions::adc(
                    &mut self.registers.a,
                    self.registers.b,
                    &mut self.registers.f,
                );
                4
            }
            0x89 => {
                Instructions::adc(
                    &mut self.registers.a,
                    self.registers.c,
                    &mut self.registers.f,
                );
                4
            }
            0x8A => {
                Instructions::adc(
                    &mut self.registers.a,
                    self.registers.d,
                    &mut self.registers.f,
                );
                4
            }
            0x8B => {
                Instructions::adc(
                    &mut self.registers.a,
                    self.registers.e,
                    &mut self.registers.f,
                );
                4
            }
            0x8C => {
                Instructions::adc(
                    &mut self.registers.a,
                    self.registers.h,
                    &mut self.registers.f,
                );
                4
            }
            0x8D => {
                Instructions::adc(
                    &mut self.registers.a,
                    self.registers.l,
                    &mut self.registers.f,
                );
                4
            }
            0x8E => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::adc(&mut self.registers.a, v, &mut self.registers.f);
                8
            }
            0x8F => {
                let v: u8 = self.registers.a;

                Instructions::adc(&mut self.registers.a, v, &mut self.registers.f);
                4
            }
            0x90 => {
                Instructions::sub(
                    &mut self.registers.a,
                    self.registers.b,
                    &mut self.registers.f,
                );
                4
            }
            0x91 => {
                Instructions::sub(
                    &mut self.registers.a,
                    self.registers.c,
                    &mut self.registers.f,
                );
                4
            }
            0x92 => {
                Instructions::sub(
                    &mut self.registers.a,
                    self.registers.d,
                    &mut self.registers.f,
                );
                4
            }
            0x93 => {
                Instructions::sub(
                    &mut self.registers.a,
                    self.registers.e,
                    &mut self.registers.f,
                );
                4
            }
            0x94 => {
                Instructions::sub(
                    &mut self.registers.a,
                    self.registers.h,
                    &mut self.registers.f,
                );
                4
            }
            0x95 => {
                Instructions::sub(
                    &mut self.registers.a,
                    self.registers.l,
                    &mut self.registers.f,
                );
                4
            }
            0x96 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::sub(&mut self.registers.a, v, &mut self.registers.f);
                8
            }
            0x97 => {
                let v: u8 = self.registers.a;

                Instructions::sub(&mut self.registers.a, v, &mut self.registers.f);
                4
            }
            0x98 => {
                Instructions::sbc(
                    &mut self.registers.a,
                    self.registers.b,
                    &mut self.registers.f,
                );
                4
            }
            0x99 => {
                Instructions::sbc(
                    &mut self.registers.a,
                    self.registers.c,
                    &mut self.registers.f,
                );
                4
            }
            0x9A => {
                Instructions::sbc(
                    &mut self.registers.a,
                    self.registers.d,
                    &mut self.registers.f,
                );
                4
            }
            0x9B => {
                Instructions::sbc(
                    &mut self.registers.a,
                    self.registers.e,
                    &mut self.registers.f,
                );
                4
            }
            0x9C => {
                Instructions::sbc(
                    &mut self.registers.a,
                    self.registers.h,
                    &mut self.registers.f,
                );
                4
            }
            0x9D => {
                Instructions::sbc(
                    &mut self.registers.a,
                    self.registers.l,
                    &mut self.registers.f,
                );
                4
            }
            0x9E => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let mut v: u8 = mem.read_u8(address as usize);

                Instructions::sbc(&mut self.registers.a, v, &mut self.registers.f);
                8
            }
            0x9F => {
                let a = self.registers.a;

                Instructions::sbc(&mut self.registers.a, a, &mut self.registers.f);
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
            0xA8 => {
                Instructions::xor(
                    &mut self.registers.a,
                    &mut self.registers.f,
                    self.registers.b,
                );
                4
            }
            0xA9 => {
                Instructions::xor(
                    &mut self.registers.a,
                    &mut self.registers.f,
                    self.registers.c,
                );
                4
            }
            0xAA => {
                Instructions::xor(
                    &mut self.registers.a,
                    &mut self.registers.f,
                    self.registers.d,
                );
                4
            }
            0xAB => {
                Instructions::xor(
                    &mut self.registers.a,
                    &mut self.registers.f,
                    self.registers.e,
                );
                4
            }
            0xAC => {
                Instructions::xor(
                    &mut self.registers.a,
                    &mut self.registers.f,
                    self.registers.h,
                );
                4
            }
            0xAD => {
                Instructions::xor(
                    &mut self.registers.a,
                    &mut self.registers.f,
                    self.registers.l,
                );
                4
            }
            0xAE => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::xor(&mut self.registers.a, &mut self.registers.f, v);
                8
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
            0xB8 => {
                Instructions::cp(self.registers.a, self.registers.b, &mut self.registers.f);
                4
            }
            0xB9 => {
                Instructions::cp(self.registers.a, self.registers.c, &mut self.registers.f);
                4
            }
            0xBA => {
                Instructions::cp(self.registers.a, self.registers.d, &mut self.registers.f);
                4
            }
            0xBB => {
                Instructions::cp(self.registers.a, self.registers.e, &mut self.registers.f);
                4
            }
            0xBC => {
                Instructions::cp(self.registers.a, self.registers.h, &mut self.registers.f);
                4
            }
            0xBD => {
                Instructions::cp(self.registers.a, self.registers.l, &mut self.registers.f);
                4
            }
            0xBE => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);
                let v: u8 = mem.read_u8(address as usize);

                Instructions::cp(self.registers.a, v, &mut self.registers.f);
                8
            }
            0xBF => {
                Instructions::cp(self.registers.a, self.registers.a, &mut self.registers.f);
                4
            }
            0xC0 => {
                if !self.registers.f.zero {
                    let address: u16 = self.pop_word(mem);

                    self.registers.pc = address;
                    return 20;
                }
                8
            }
            0xC1 => {
                let v: u16 = self.pop_word(mem);
                let v8s: (u8, u8) = BinUtils::u8s_from_u16(v);

                self.registers.b = v8s.0;
                self.registers.c = v8s.1;
                12
            }
            0xC2 => {
                let dw: u16 = self.fetch_u16(mem);

                if !self.registers.f.zero {
                    self.registers.pc = dw;
                    return 16;
                }
                12
            }
            0xC3 => {
                let address: u16 = self.fetch_u16(mem);

                self.registers.pc = address;
                12
            }
            0xC4 => {
                let dw: u16 = self.fetch_u16(mem);

                if !self.registers.f.zero {
                    self.push_word(mem, self.registers.pc);
                    self.registers.pc = dw;
                    return 24;
                }
                12
            }
            0xC5 => {
                let word: u16 = BinUtils::u16_from_u8s(self.registers.b, self.registers.c);

                self.push_word(mem, word);
                16
            }
            0xC6 => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::add(&mut self.registers.a, v, &mut self.registers.f);
                8
            }
            0xC7 => {
                self.push_word(mem, self.registers.pc);
                self.registers.pc = 0;
                32
            }
            0xC8 => {
                if self.registers.f.zero {
                    let address: u16 = self.pop_word(mem);

                    self.registers.pc = address;
                    return 20;
                }
                8
            }
            0xC9 => {
                let address: u16 = self.pop_word(mem);

                self.registers.pc = address;
                16
            }
            0xCA => {
                let dw: u16 = self.fetch_u16(mem);

                if self.registers.f.zero {
                    self.registers.pc = dw;
                    return 16;
                }
                12
            }
            0xCB => {
                let extended_op_code: u8 = self.fetch_u8(mem);

                self.call_extended(mem, extended_op_code)
            }
            0xCC => {
                let dw: u16 = self.fetch_u16(mem);

                if self.registers.f.zero {
                    self.push_word(mem, self.registers.pc);
                    self.registers.pc = dw;
                    return 24;
                }
                12
            }
            0xCD => {
                let dw: u16 = self.fetch_u16(mem);

                self.push_word(mem, self.registers.pc);
                self.registers.pc = dw;
                12
            }
            0xCE => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::adc(&mut self.registers.a, v, &mut self.registers.f);
                8
            }
            0xCF => {
                self.push_word(mem, self.registers.pc);
                self.registers.pc = 0x08;
                32
            }
            0xD0 => {
                if !self.registers.f.carry {
                    let address: u16 = self.pop_word(mem);

                    self.registers.pc = address;
                    return 20;
                }
                8
            }
            0xD1 => {
                let w: u16 = self.pop_word(mem);
                let u8s = BinUtils::u8s_from_u16(w);

                self.registers.d = u8s.0;
                self.registers.e = u8s.1;
                12
            }
            0xD2 => {
                let dw: u16 = self.fetch_u16(mem);

                if !self.registers.f.carry {
                    self.registers.pc = dw;
                    return 16;
                }
                12
            }
            0xD4 => {
                let dw: u16 = self.fetch_u16(mem);

                if !self.registers.f.carry {
                    self.push_word(mem, self.registers.pc);
                    self.registers.pc = dw;
                    return 24;
                }
                12
            }
            0xD5 => {
                let v: u16 = BinUtils::u16_from_u8s(self.registers.d, self.registers.e);

                self.push_word(mem, v);
                16
            }
            0xD6 => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::sub(&mut self.registers.a, v, &mut self.registers.f);
                8
            }
            0xD7 => {
                self.push_word(mem, self.registers.pc);
                self.registers.pc = 0x10;
                32
            }
            0xD8 => {
                if self.registers.f.carry {
                    let address: u16 = self.pop_word(mem);

                    self.registers.pc = address;
                    return 20;
                }
                8
            }
            0xD9 => {
                let address: u16 = self.pop_word(mem);

                self.registers.pc = address;
                // TODO: Enable interrupts
                8
            }
            0xDA => {
                let dw: u16 = self.fetch_u16(mem);

                if self.registers.f.carry {
                    self.registers.pc = dw;
                    return 16;
                }
                12
            }
            0xDC => {
                let dw: u16 = self.fetch_u16(mem);

                if self.registers.f.carry {
                    self.push_word(mem, self.registers.pc);
                    self.registers.pc = dw;
                    return 24;
                }
                12
            }
            0xDE => {
                let dv: u8 = self.fetch_u8(mem);

                Instructions::sbc(&mut self.registers.a, dv, &mut self.registers.f);
                8
            }
            0xDF => {
                self.push_word(mem, self.registers.pc);
                self.registers.pc = 0x18;
                32
            }
            0xE0 => {
                let dv: u8 = self.fetch_u8(mem);
                let address: u16 = 0xFF00 | dv as u16;

                mem.write_u8(self.registers.a, address as usize);
                12
            }
            0xE1 => {
                let w: u16 = self.pop_word(mem);
                let u8s = BinUtils::u8s_from_u16(w);

                self.registers.h = u8s.0;
                self.registers.l = u8s.1;
                12
            }
            0xE2 => {
                mem.write_u8(self.registers.a, 0xFF00 | (self.registers.c as usize));
                8
            }
            0xE5 => {
                let v: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);

                self.push_word(mem, v);
                16
            }
            0xE6 => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::and(&mut self.registers.a, &mut self.registers.f, v);
                8
            }
            0xE7 => {
                self.push_word(mem, self.registers.pc);
                self.registers.pc = 0x20;
                32
            }
            0xE8 => {
                let v: u16 = self.fetch_u8(mem) as i8 as i16 as u16;

                // TODO: Use mixed_integer_ops when available
                self.registers.f.half_carry = (self.registers.sp & 0x000F) + (v & 0x000F) > 0x000F;
                self.registers.f.carry = (self.registers.sp & 0x00FF) + (v & 0x00FF) > 0x00FF;
                self.registers.sp = self.registers.sp.wrapping_add(v);
                self.registers.f.zero = false;
                self.registers.f.substract = false;
                16
            }
            0xE9 => {
                let address: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);

                self.registers.pc = address;
                4
            }
            0xEA => {
                let address: u16 = self.fetch_u16(mem);

                // println!("W {:x?} = {:x?}", address, self.registers.a);
                mem.write_u8(self.registers.a, address as usize);
                16
            }
            0xEE => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::xor(&mut self.registers.a, &mut self.registers.f, v);
                8
            }
            0xEF => {
                self.push_word(mem, self.registers.pc);
                self.registers.pc = 0x28;
                32
            }
            0xF0 => {
                let dv: u8 = self.fetch_u8(mem);
                let address: u16 = 0xFF00 | dv as u16;

                self.registers.a = mem.read_u8(address as usize);
                12
            }
            0xF1 => {
                let w: u16 = self.pop_word(mem);
                let u8s = BinUtils::u8s_from_u16(w);

                self.registers.a = u8s.0;
                self.registers.f = FRegister::from(u8s.1);
                12
            }
            0xF2 => {
                let address: u16 = 0xFF00 | self.registers.c as u16;

                self.registers.a = mem.read_u8(address as usize);
                8
            }
            0xF3 => {
                self.imd_next = true;
                println!("IME disabled on next instruction");
                4
            }
            0xF5 => {
                let v: u16 = BinUtils::u16_from_u8s(self.registers.a, u8::from(self.registers.f));

                self.push_word(mem, v);
                16
            }
            0xF6 => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::or(&mut self.registers.a, &mut self.registers.f, v);
                8
            }
            0xF7 => {
                self.push_word(mem, self.registers.pc);
                self.registers.pc = 0x30;
                32
            }
            0xF8 => {
                let v: u16 = self.fetch_u8(mem) as i8 as i16 as u16;
                let result: u16 = self.registers.sp.wrapping_add(v);
                let rsplit: (u8, u8);

                self.registers.f.substract = false;
                self.registers.f.zero = false;
                self.registers.f.half_carry = (self.registers.sp & 0x000F) + (v & 0x000F) > 0x000F;
                self.registers.f.carry = (self.registers.sp & 0x00FF) + (v & 0x00FF) > 0x00FF;
                rsplit = BinUtils::u8s_from_u16(result);
                self.registers.h = rsplit.0;
                self.registers.l = rsplit.1;
                12
            }
            0xF9 => {
                let hl: u16 = BinUtils::u16_from_u8s(self.registers.h, self.registers.l);

                self.registers.sp = hl;
                8
            }
            0xFA => {
                let address: u16 = self.fetch_u16(mem);
                let v: u8 = mem.read_u8(address as usize);

                // println!("R {:x?} = {:x?}", address, v);
                self.registers.a = v;
                16
            }
            0xFB => {
                self.ime_next = true;
                println!("IME enabled on next step");
                4
            }
            0xFE => {
                let v: u8 = self.fetch_u8(mem);

                Instructions::cp(self.registers.a, v, &mut self.registers.f);
                8
            }
            0xFF => {
                self.push_word(mem, self.registers.pc);
                self.registers.pc = 0x38;
                32
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

        /* println!("{:x?}: {:x?}                            A:{:x?} F:{:x?} B:{:x?} C:{:x?} D:{:x?} E:{:x?} H:{:x?} L:{:x?} LY:{:x?} SP:{:x?}",
            self.registers.pc - 1,
            inst_op_code,
            self.registers.a,
            u8::from(self.registers.f),
            self.registers.b,
            self.registers.c,
            self.registers.d,
            self.registers.e,
            self.registers.h,
            self.registers.l,
            mem.ly,
            self.registers.sp

        ); */
        self.call(mem, inst_op_code) as usize
    }
}

struct Instructions {}
impl Instructions {
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
        f_reg.half_carry = (*reg & 0x0F) == 0;
        *reg = (*reg).wrapping_sub(1);
        f_reg.zero = *reg == 0;
        f_reg.substract = true;
    }

    pub fn dec_nn(high_reg: &mut u8, low_reg: &mut u8) {
        let v: u16 = BinUtils::u16_from_u8s(*high_reg, *low_reg);
        let vs: (u8, u8) = BinUtils::u8s_from_u16(v.wrapping_sub(1));

        *high_reg = vs.0;
        *low_reg = vs.1;
    }

    pub fn bit(bit: u8, reg: u8, f_reg: &mut FRegister) {
        f_reg.zero = reg & (1 << (bit as u32)) == 0;
        f_reg.substract = false;
        f_reg.half_carry = true;
    }

    pub fn set(bit: u8, reg: &mut u8) {
        *reg |= 1 << bit;
    }

    pub fn jr_n(offset: i8, pc: &mut u16) {
        *pc = ((*pc as u32 as i32) + (offset as i32)) as u16;
    }

    pub fn inc(reg: &mut u8, f_reg: &mut FRegister) {
        let res: u8 = (*reg).wrapping_add(1);

        f_reg.zero = res == 0;
        f_reg.substract = false;
        f_reg.half_carry = (*reg & 0x0F) + 1 > 0x0F;
        *reg = res;
    }

    pub fn inc_nn(high_reg: &mut u8, low_reg: &mut u8) {
        let v: u16 = BinUtils::u16_from_u8s(*high_reg, *low_reg);
        let vs: (u8, u8) = BinUtils::u8s_from_u16(v.wrapping_add(1));

        *high_reg = vs.0;
        *low_reg = vs.1;
    }

    pub fn swap(n: &mut u8, f_reg: &mut FRegister) {
        f_reg.zero = *n == 0;
        f_reg.substract = false;
        f_reg.half_carry = false;
        f_reg.carry = false;
        *n = (*n >> 4) | (*n << 4);
    }

    pub fn rr(reg: &mut u8, f_reg: &mut FRegister) {
        let carry: bool = *reg & 0x1 == 0x1;

        *reg = (*reg >> 1) | (if f_reg.carry { 0x80 } else { 0 });
        f_reg.zero = *reg == 0;
        f_reg.substract = false;
        f_reg.half_carry = false;
        f_reg.carry = carry;
    }

    pub fn rrc(reg: &mut u8, f_reg: &mut FRegister, with_carry: bool) {
        let carry: bool = *reg & 0x1 == 0x1;

        *reg = (*reg >> 1) | (if carry && with_carry { 0x80 } else { 0 });
        f_reg.zero = *reg == 0;
        f_reg.substract = false;
        f_reg.half_carry = false;
        f_reg.carry = carry;
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

    pub fn sbc(a_reg: &mut u8, v: u8, f_reg: &mut FRegister) {
        let carry: u8 = if f_reg.carry { 1 } else { 0 };
        let result: u8 = a_reg.wrapping_sub(v).wrapping_sub(carry);

        f_reg.zero = result == 0;
        f_reg.substract = true;
        f_reg.carry = (*a_reg as u16) < (v as u16) + (carry as u16);
        f_reg.half_carry = (*a_reg & 0xF) < ((v & 0xF) + carry);
        *a_reg = result;
    }

    pub fn sla(a_reg: &mut u8, f_reg: &mut FRegister) {
        f_reg.substract = false;
        f_reg.half_carry = false;
        f_reg.carry = *a_reg & (1 << 7) != 0;
        *a_reg <<= 1;
        f_reg.zero = *a_reg == 0;
    }

    pub fn sra(a_reg: &mut u8, f_reg: &mut FRegister) {
        f_reg.substract = false;
        f_reg.half_carry = false;
        f_reg.carry = *a_reg & 1 != 0;
        // f_reg.carry = false;
        *a_reg = *a_reg >> 1 | *a_reg & 0x80;
        f_reg.zero = *a_reg == 0;
    }

    // TODO: Check HC and C flags
    pub fn add(a_reg: &mut u8, v: u8, f_reg: &mut FRegister) {
        let result: u8 = a_reg.wrapping_add(v);

        f_reg.zero = result == 0;
        f_reg.substract = false;
        f_reg.half_carry = ((*a_reg & 0xF) + (v & 0xF)) > 0xF;
        f_reg.carry = (*a_reg as u16) + (v as u16) > 0xFF;
        *a_reg = result;
    }

    pub fn add_nn(h: &mut u8, l: &mut u8, v: u16, f_reg: &mut FRegister) {
        let hl: u16 = BinUtils::u16_from_u8s(*h, *l);
        let hl_a: u16 = hl.wrapping_add(v);
        let hlu8s: (u8, u8) = BinUtils::u8s_from_u16(hl_a);

        f_reg.substract = false;
        f_reg.carry = hl > (0xFFFF - v);
        f_reg.half_carry = (hl & 0x07FF) + (v & 0x07FF) > 0x07FF;
        *h = hlu8s.0;
        *l = hlu8s.1;
    }

    pub fn res(r: &mut u8, b: u8) {
        *r &= !(1u8 << b);
    }

    pub fn adc(a_reg: &mut u8, n: u8, f_reg: &mut FRegister) {
        let carry: u8 = if f_reg.carry { 1 } else { 0 };
        let res: u8 = a_reg.wrapping_add(n).wrapping_add(carry);
        let res_l: usize = *a_reg as usize + n as usize + carry as usize;

        f_reg.zero = res == 0;
        f_reg.substract = false;
        f_reg.half_carry = (*a_reg & 0xF) + (n & 0xF) + carry > 0xF;
        f_reg.carry = res_l > 255;
        *a_reg = res;
    }

    pub fn srl(reg: &mut u8, f_reg: &mut FRegister) {
        f_reg.carry = *reg & 0x1 == 0x1;
        let res: u8 = *reg >> 1;

        f_reg.zero = res == 0;
        f_reg.substract = false;
        f_reg.half_carry = false;
        *reg = res;
    }
}

// TODO: Move to another file
impl Cpu {
    // Load 16bits value to a 16bits register
}

#[test]
fn dec_hc() {
    let mut a: u8 = 0x40;
    let mut f: FRegister = FRegister {
        zero: true,
        substract: false,
        half_carry: false,
        carry: false,
    };

    Instructions::dec(&mut a, &mut f);
    assert_eq!(a, 0x3F);
    assert!(!f.zero);
    assert!(f.substract);
    assert!(!f.carry);
    assert!(f.half_carry);
}

#[test]
fn dec_no_hc() {
    let mut a: u8 = 0x4;
    let mut f: FRegister = FRegister {
        zero: true,
        substract: false,
        half_carry: false,
        carry: false,
    };

    Instructions::dec(&mut a, &mut f);
    assert_eq!(a, 3);
    assert!(!f.zero);
    assert!(f.substract);
    assert!(!f.carry);
    assert!(!f.half_carry);
}
