pub mod cpu;
pub mod memory;
pub mod registers;

use cpu::Cpu;
use memory::Memory;

pub struct Gameboy {
    boot: &'static [u8; 256],
    cpu: Cpu,
    mem_map: Memory,
}

impl Gameboy {
    pub fn new() -> Self {
        Gameboy {
            boot: include_bytes!("../../res/boot.bin"),
            cpu: Cpu::new(),
            mem_map: Memory::new(),
        }
    }

    pub fn power_up(&mut self) {
        self.cpu.registers.pc = 0x0;
        self.mount_boot();
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
