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
    boot: &'static [u8; 256],
    rom: [u8; 0x8000],
    vram: [u8; 0x2000],

    pub is_booting: bool,
    // Special Registers
    //   Sound
    nr11: u8,
    //   Graphics
    ly: u8,
    scy: u8,
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

            is_booting: false,

            // Memory
            boot: include_bytes!("../../res/boot.bin"),
            rom: [0; 0x8000],
            vram: [0; 0x2000],

            // Special registers
            nr11: 0,
            ly: 0x90, // TODO: This should be 0 on startup, this is set so that the boot sequence doesn't loop forever
            scy: 0,
        }
    }

    fn write_io_u8(&mut self, value: u8, address: usize) {
        match address {
            0xFF11 => self.nr11 = value,
            0xFF42 => self.scy = value,
            0xFF44 => self.ly = value,
            _ => {}
        }
    }

    fn read_io_u8(&mut self, address: usize) -> u8 {
        match address {
            0xFF11 => self.nr11,
            0xFF42 => self.scy,
            0xFF44 => self.ly,
            _ => panic!("Unknown IO register: {:#02x}", address),
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
            if self.is_booting && address <= 0xFF {
                // During boot, any read from value 0x0 to 0xFF is redirected to the boot rom
                return self.boot[address];
            }
            return self.rom[address - self.rom_address_bound.start];
        } else {
            panic!("{:#02x} is not an implemented memory address", address);
        }
    }
}
