use super::MemoryBankController;

pub struct MBC0 {
    pub rom: [u8; 0x8000],
}

impl MBC0 {
    pub fn new(buffer: Vec<u8>) -> Self {
        let mut mbc: MBC0 = MBC0 { rom: [0x0; 0x8000] };

        if buffer.len() > 0x8000 {
            panic!("Got too much data for a MBC of type 0.");
        }
        mbc.rom[..buffer.len()].copy_from_slice(&buffer[..]);
        mbc
    }
}

impl MemoryBankController for MBC0 {
    fn read_u8(&self, address: usize) -> u8 {
        self.rom[address as usize]
    }

    fn write_u8(&mut self, address: usize, v: u8) {
        self.rom[address as usize] = v;
    }
}
