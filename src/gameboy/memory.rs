use std::ops::Range;

pub struct Memory {
    // Memory bounds
    rom_address_bound: Range<usize>,
    vram_address_bound: Range<usize>,
    sram8_address_bound: Range<usize>,
    iram8_address_bound: Range<usize>,
    iram8_echo_address_bound: Range<usize>,
    sam_address_bound: Range<usize>,
    io_address_bound: Range<usize>,
    iram_address_bound: Range<usize>,

    // Memory
    rom: [u8; 0x8000],
    vram: [u8; 0x2000],

    // Special Registers
    nr11: u8,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            // Memory bounds
            rom_address_bound: Range {
                start: 0x0000,
                end: 0x8000,
            },
            vram_address_bound: Range {
                start: 0x8000,
                end: 0xA000,
            },
            sram8_address_bound: Range {
                start: 0xA000,
                end: 0xC000,
            },
            iram8_address_bound: Range {
                start: 0xC000,
                end: 0xE000,
            },
            iram8_echo_address_bound: Range {
                start: 0xE000,
                end: 0xFE00,
            },
            sam_address_bound: Range {
                start: 0xFE00,
                end: 0xFEA0,
            },
            io_address_bound: Range {
                start: 0xFF00,
                end: 0xFF4C,
            },
            iram_address_bound: Range {
                start: 0xFF80,
                end: 0xFFFF,
            },

            // Memory
            rom: [0; 0x8000],
            vram: [0; 0x2000],

            // Special registers
            nr11: 0,
        }
    }

    fn write_io_u8(&mut self, value: u8, address: usize) {
        match address {
            0xFF11 => self.nr11 = value,
            _ => {}
        }
    }

    pub fn write_u8(&mut self, value: u8, address: usize) {
        if self.rom_address_bound.contains(&address) {
            self.rom[address - self.rom_address_bound.start] = value;
        } else if self.vram_address_bound.contains(&address) {
            self.vram[address - self.vram_address_bound.start] = value;
        } else if self.io_address_bound.contains(&address) {
            self.write_io_u8(value, address);
        } else {
            panic!("{:#02x} is not an implemented memory address", address);
        }
    }

    pub fn read_u8(&mut self, address: usize) -> u8 {
        if self.rom_address_bound.contains(&address) {
            return self.rom[address - self.rom_address_bound.start];
        } else {
            panic!("{:#02x} is not an implemented memory address", address);
        }
    }
}
