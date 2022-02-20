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
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            // Memory bounds
            rom_address_bound: Range {
                start: 0x0000,
                end: 0x7FFF,
            },
            vram_address_bound: Range {
                start: 0x8000,
                end: 0x9FFF,
            },
            sram8_address_bound: Range {
                start: 0xA000,
                end: 0xBFFF,
            },
            iram8_address_bound: Range {
                start: 0xC000,
                end: 0xDFFF,
            },
            iram8_echo_address_bound: Range {
                start: 0xE000,
                end: 0xFDFF,
            },
            sam_address_bound: Range {
                start: 0xFE00,
                end: 0xFE9F,
            },
            io_address_bound: Range {
                start: 0xFF00,
                end: 0xFF4B,
            },
            iram_address_bound: Range {
                start: 0xFF80,
                end: 0xFFFE,
            },

            // Memory
            rom: [0; 0x8000],
        }
    }

    pub fn write_u8(&mut self, value: u8, address: usize) {
        if self.rom_address_bound.contains(&address) {
            self.rom[address - self.rom_address_bound.start] = value;
        } else {
            panic!("unimplemented memory address");
        }
    }

    pub fn read_u8(&mut self, address: usize) -> u8 {
        if self.rom_address_bound.contains(&address) {
            return self.rom[address - self.rom_address_bound.start];
        } else {
            panic!("unimplemented memory address");
        }
    }
}
