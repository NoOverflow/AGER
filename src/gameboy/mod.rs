pub mod cpu;
pub mod registers;

use cpu::Cpu;

pub struct Gameboy {
    cpu: Cpu,
}

impl Gameboy {
    pub fn new() -> Self {
        Gameboy { cpu: Cpu::new() }
    }

    pub fn dump_bios(&self) {
        self.cpu.dump_bios();
    }
}
