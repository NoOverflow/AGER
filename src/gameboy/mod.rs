pub mod bin_utils;
pub mod cpu;
pub mod mbc;
pub mod memory;
pub mod registers;

use cpu::Cpu;
use mbc::mbc0::MBC0;
use memory::Memory;

use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Gameboy {
    cpu: Cpu,
    mem_map: Memory,

    stop: bool,
}

impl Gameboy {
    pub fn new() -> Self {
        Gameboy {
            cpu: Cpu::new(),
            mem_map: Memory::new(),
            stop: false,
        }
    }

    pub fn load_cartridge(&mut self, path: &str) {
        let mut f_handle = File::open(path).unwrap();
        let mut buffer = Vec::new();

        f_handle.read_to_end(&mut buffer).unwrap();
        if buffer.len() < 0x100 {
            panic!("GB File must be at least 256 bytes.");
        }
        self.mem_map.rom = match buffer[0x147] {
            // TODO: Move to an enum
            0x0 => Box::new(MBC0::new(buffer)),
            _ => panic!(
                "Game requires a MBC of type {} which is not yet implemented.",
                buffer[0x147]
            ),
        }
    }

    pub fn power_up(&mut self) {
        self.cpu.registers.pc = 0x0;
        self.cpu.registers.sp = 0xFFFE;
        self.mem_map.write_io_u8(0x01, 0xFF50);
    }

    pub fn cycle(&mut self) {
        if !self.stop {
            self.cpu.cycle(&mut self.mem_map);
        }
        if self.cpu.registers.pc == 0x100 && !self.stop {
            println!("Boot ROM is done. We're now in cartridge territory.");
            self.stop = true;
        }
    }
}
