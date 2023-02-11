pub mod bin_utils;
pub mod cpu;
pub mod debugger;
pub mod gpu;
pub mod mbc;
pub mod memory;
pub mod registers;
pub mod timer;

use cpu::Cpu;
use gpu::Gpu;
use mbc::mbc0::MBC0;
use mbc::mbc1::MBC1;
use memory::Memory;
use registers::FRegister;
use std::fs::File;
use std::io::prelude::*;
use timer::Timer;

pub struct Gameboy {
    pub cpu: Cpu,
    pub gpu: Gpu,
    pub mem_map: Memory,
    pub timer: Timer,
}

impl Gameboy {
    pub fn new() -> Self {
        Gameboy {
            cpu: Cpu::new(),
            gpu: Gpu::new(),
            mem_map: Memory::new(),
            timer: Timer::new(),
        }
    }

    pub fn get_screen_buffer(&self) -> Vec<u32> {
        self.gpu.get_screen_buffer(&self.mem_map)
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
            0x1 => Box::new(MBC1::new(buffer)),
            _ => panic!(
                "Game requires a MBC of type {} which is not yet implemented.",
                buffer[0x147]
            ),
        }
    }

    pub fn power_up(&mut self) {
        self.cpu.registers.pc = 0x100;
        self.cpu.registers.f = FRegister::from(0xB0);
        self.cpu.registers.a = 0x1;
        self.cpu.registers.b = 0x0;
        self.cpu.registers.c = 0x13;
        self.cpu.registers.d = 0x0;
        self.cpu.registers.e = 0xd8;
        self.cpu.registers.h = 0x01;
        self.cpu.registers.l = 0x4d;

        self.mem_map.boot_rom_disable = 1;

        self.cpu.registers.sp = 0xFFFE;
    }

    pub fn clear_interrupts(&mut self) {
        self.mem_map.iflag = memory::Ei::from(0);
    }

    pub fn check_interrupts(&mut self) {
        let mut int_address: u16 = 0;

        if !self.cpu.ime {
            self.clear_interrupts();
            return 0;
        }

        if self.mem_map.iflag.timer_overflow && self.mem_map.ei.timer_overflow {
            println!("Timer interrupt.");
            int_address = 0x50;
            self.mem_map.iflag.timer_overflow = false;
        }

        if self.mem_map.iflag.lcdc && self.mem_map.ei.lcdc {
            println!("LCDC Interrupt");
            int_address = 0x48;
            self.mem_map.iflag.lcdc = false;
        }

        if self.mem_map.iflag.vblank && self.mem_map.ei.vblank {
            println!("VBlank Interrupt");
            int_address = 0x40;
            self.mem_map.halted = false;
        }

        self.clear_interrupts();
        if int_address == 0 {
            return;
        }
        self.cpu.ime = false;
        self.cpu.push_word(&mut self.mem_map, self.cpu.registers.pc);
        self.cpu.registers.pc = int_address;
        10
    }

    pub fn timer_cycle(&mut self, cycles: usize) {
        if self.timer.increment_tima(&mut self.mem_map, cycles) {
            self.mem_map.iflag.timer_overflow = true;
        }
    }

    pub fn cycle(&mut self, delta: u64) {
        let fps_interval: f64 = 1f64 / (60f64 + (delta as f64 / 100f64)) as f64; // Sleep time in s
        let gb_freq = 4.194304 * 1_000_000.0; // in Hz
        let clk_per_frame = (gb_freq as f64) * fps_interval as f64;
        let mut spent_cycles: usize = 0;

        while (spent_cycles as f64) < clk_per_frame {
            let mut cpu_cycles: usize = 0;

            // Delay interrupt master enable by one instruction
            if self.cpu.ime_next {
                self.cpu.ime = true;
                self.cpu.ime_next = false;
            }
            if self.cpu.imd_next {
                self.cpu.ime = false;
                self.cpu.imd_next = false;
            }
            cpu_cycles += self.check_interrupts();
            if !self.mem_map.halted {
                cpu_cycles += self.cpu.cycle(&mut self.mem_map);
                self.timer.increment_div(&mut self.mem_map, cpu_cycles);
            } else {
                cpu_cycles += 4;
            }

            self.timer_cycle(cpu_cycles);
            if !self.mem_map.stopped {
                self.gpu.cycle(&mut self.mem_map, cpu_cycles);
            }
            spent_cycles += cpu_cycles;
        }
    }
}
