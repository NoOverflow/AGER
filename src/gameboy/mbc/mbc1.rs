use super::MemoryBankController;
use std::ops::Range;

pub struct MBC1 {
    // Ranges
    pub rom_bank_0_range: Range<usize>,
    pub rom_banks_range: Range<usize>,
    pub ram_banks_range: Range<usize>,
    pub ram_enable_range: Range<usize>,
    pub rom_bank_number_range: Range<usize>,
    pub ram_romupbits_bank_number_range: Range<usize>,
    pub rom_ram_mode_select_range: Range<usize>,

    pub rom_bank_low: u8,
    pub rom_bank_high: u8,

    pub ram_bank: u8,

    pub rom: [u8; 0x20000],
    pub ram: [u8; 0x8000],

    pub mode: bool, // 0=ROM Bank mode 1=RAM Bank mode
    pub ram_enable: bool,
}

impl MBC1 {
    pub fn new(buffer: Vec<u8>) -> Self {
        let mut mbc: MBC1 = MBC1 {
            rom_bank_0_range: Range {
                start: 0x0000,
                end: 0x4000,
            },
            rom_banks_range: Range {
                start: 0x4000,
                end: 0x8000,
            },
            ram_banks_range: Range {
                start: 0xA000,
                end: 0xC000,
            },
            ram_enable_range: Range {
                start: 0x0000,
                end: 0x2000,
            },
            rom_bank_number_range: Range {
                start: 0x2000,
                end: 0x4000,
            },
            ram_romupbits_bank_number_range: Range {
                start: 0x4000,
                end: 0x6000,
            },
            rom_ram_mode_select_range: Range {
                start: 0x6000,
                end: 0x8000,
            },

            rom_bank_low: 0x1,
            rom_bank_high: 0x0,

            ram_bank: 0x0,

            rom: [0x0; 0x20000],
            ram: [0x0; 0x8000],

            mode: false, // 0=ROM Bank mode 1=RAM Bank mode
            ram_enable: false,
        };

        if buffer.len() > 0x20000 {
            panic!("Got too much data for a MBC of type 1.");
        }
        for i in 0..buffer.len() {
            mbc.rom[i] = buffer[i];
        }
        return mbc;
    }

    fn get_bank_index(&self) -> u8 {
        self.rom_bank_high | self.rom_bank_low
    }
}

impl MemoryBankController for MBC1 {
    fn read_u8(&self, address: usize) -> u8 {
        if self.rom_bank_0_range.contains(&address) {
            self.rom[address]
        } else if self.ram_banks_range.contains(&address) {
            self.ram[(address - self.ram_banks_range.start) + self.ram_bank as usize * 0x2000]
        } else if self.rom_banks_range.contains(&address) {
            self.rom
                [(address - self.rom_banks_range.start) + (self.get_bank_index() as usize * 0x4000)]
        } else {
            panic!("Unknown MBC1 read address {:#02x}", address);
        }
    }

    fn write_u8(&mut self, address: usize, v: u8) {
        if self.ram_banks_range.contains(&address) {
            self.ram[(address - self.ram_banks_range.start) + self.ram_bank as usize * 0x2000] = v;
        } else if self.ram_enable_range.contains(&address) {
            self.ram_enable = v & 0x0F == 0x0A;
        } else if self.rom_bank_number_range.contains(&address) {
            self.rom_bank_low = v & 0x1F;
            if self.rom_bank_low == 0x0 {
                self.rom_bank_low = 0x1;
            }
        } else if self.ram_romupbits_bank_number_range.contains(&address) {
            if self.mode {
                // RAM Mode
                self.ram_bank = v & 0x3;
            } else {
                // ROM Mode
                self.rom_bank_high = (v & 0x3) << 6;
            }
        } else if self.rom_ram_mode_select_range.contains(&address) {
            self.mode = v != 0;
            if !self.mode {
                self.ram_bank = 0x0;
            }
        } else {
            panic!("Unknown MBC1 write address {:#02x}", address);
        }
    }
}
