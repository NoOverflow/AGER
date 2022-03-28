pub mod mbc0;
pub mod mbc1;

pub enum MbcId {
    MBC0 = 0x0,
    MBC1 = 0x1,
    MBC1WRAM = 0x2,
    // TODO: Finish
}

pub trait MemoryBankController {
    fn read_u8(&self, address: usize) -> u8;
    fn write_u8(&mut self, address: usize, v: u8);
}
