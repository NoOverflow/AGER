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
        Gameboy { cpu: Cpu::new() }
    }

    pub fn dump_bios(&self) {
        self.cpu.dump_bios();
    }
}
