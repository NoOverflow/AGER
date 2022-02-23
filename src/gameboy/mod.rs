pub mod bin_utils;
pub mod cpu;
pub mod memory;
pub mod registers;

use cpu::Cpu;
use memory::Memory;

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

    pub fn power_up(&mut self) {
        self.cpu.registers.pc = 0x0;
        self.cpu.registers.sp = 0xFFFE;
        self.mem_map.is_booting = true;
    }

    pub fn cycle(&mut self) {
        self.cpu.cycle(&mut self.mem_map);
    }
}
