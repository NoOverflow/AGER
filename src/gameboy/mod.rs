pub mod cpu;
pub mod memory;
pub mod registers;

use cpu::Cpu;
use memory::Memory;

pub struct Gameboy {
    bios: &'static [u8; 256],
    cpu: Cpu,
    mem_map: Memory,
}

impl Gameboy {
    pub fn new() -> Self {
        Gameboy {
            bios: include_bytes!("../../res/bios.bin"),
            cpu: Cpu::new(),
            mem_map: Memory::new(),
        }
    }

    pub fn power_up(&mut self) {
        self.cpu.registers.pc = 0x0;
        self.mount_bios();
    }

    pub fn cycle(&mut self) {
        self.cpu.cycle(&mut self.mem_map);
    }

    fn mount_bios(&mut self) {
        for i in 0..256 {
            self.mem_map.write_u8(self.bios[i], i);
        }
    }
}
