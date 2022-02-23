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
        self.mount_boot();
        self.cpu.registers.sp = 0xFFFE;
        self.mem_map.is_booting = true;
    }

    pub fn cycle(&mut self) {
        self.cpu.cycle(&mut self.mem_map);
    }

    fn mount_boot(&mut self) {
        for i in 0..256 {
            self.mem_map.write_u8(self.boot[i], i);
        }
    }
}
