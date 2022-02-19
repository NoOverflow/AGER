pub struct Cpu {
    bios: &'static [u8; 256],
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            bios: include_bytes!("../../res/bios.bin"),
        }
    }

    pub fn dump_bios(&self) {
        for i in 0..255 {
            print!("{:#02x} ", self.bios[i]);
        }
    }
}
