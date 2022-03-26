pub mod bin_utils;
pub mod cpu;
pub mod gpu;
pub mod mbc;
pub mod memory;
pub mod registers;

use cpu::Cpu;
use gpu::Gpu;
use mbc::mbc0::MBC0;
use memory::Memory;
use minifb::{Key, Window, WindowOptions};

use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Gameboy {
    cpu: Cpu,
    gpu: Gpu,
    mem_map: Memory,
    pub stop: bool,
    test_clock: usize,
}

impl Gameboy {
    pub fn new() -> Self {
        Gameboy {
            cpu: Cpu::new(),
            gpu: Gpu::new(),
            mem_map: Memory::new(),
            stop: false,
            test_clock: 0,
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

    pub fn cycle(&mut self, buffer: &mut Vec<u32>, delta: u64) {
        let fps_interval: f64 = 1 as f64 / 60 as f64; // Sleep time in ms
        let gb_freq = 4.194304 * 1_000_000.0 as f64; // in Hz
        let clk_per_frame = (gb_freq as f64) * fps_interval as f64;
        let mut spent_cycles: usize = 0;

        while !self.stop && ((spent_cycles as f64) < clk_per_frame) {
            let cpu_cycles: usize = self.cpu.cycle(&mut self.mem_map);

            self.gpu.cycle(&mut self.mem_map, buffer, cpu_cycles);
            spent_cycles += cpu_cycles;
            if self.cpu.registers.pc == 0x100 && !self.stop {
                println!("Boot ROM is done. We're now in cartridge territory.");
                self.stop = true;
            }
        }
    }
}
