pub mod bin_utils;
pub mod cpu;
pub mod memory;
pub mod registers;

use cpu::Cpu;
use memory::Memory;

use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Gameboy {
    cpu: Cpu,
    mem_map: Memory,
}

impl Gameboy {
    pub fn new() -> Self {
        Gameboy {
            cpu: Cpu::new(),
            mem_map: Memory::new(),
        }
    }

    pub fn load_cartridge(&mut self, path: &str) {
        let mut f_handle = File::open(path).unwrap();
        let mut buffer = Vec::new();

        f_handle.read_to_end(&mut buffer).unwrap();
        if buffer.len() < 0x100 {
            panic!("GB File must be at least 256 bytes.");
        }
        for i in 0..buffer.len() {
            self.mem_map.write_u8(buffer[i], i);
        }
    }

    pub fn power_up(&mut self) {
        self.cpu.registers.pc = 0x0;
        self.cpu.registers.sp = 0xFFFE;
        self.mem_map.is_booting = true;
    }

    pub fn cycle(&mut self) {
        self.cpu.cycle(&mut self.mem_map);
    }
}
