use super::gpu::Lcdc;
use super::gpu::Stat;
use super::mbc::mbc0::MBC0;
use super::mbc::MemoryBankController;
use std::ops::Range;

#[derive(Copy, Clone)]
pub struct Ei {
    pub hi_lo: bool,
    pub serial_tx_done: bool,
    pub timer_overflow: bool,
    pub lcdc: bool,
    pub vblank: bool,
}

impl From<u8> for Ei {
    fn from(item: u8) -> Self {
        Ei {
            hi_lo: item & (1 << 4) != 0,
            serial_tx_done: item & (1 << 3) != 0,
            timer_overflow: item & (1 << 2) != 0,
            lcdc: item & (1 << 1) != 0,
            vblank: item & (1 << 0) != 0,
        }
    }
}

impl From<Ei> for u8 {
    fn from(item: Ei) -> Self {
        let mut ret: u8 = 0;

        if item.hi_lo {
            ret |= 1 << 4;
        }
        if item.serial_tx_done {
            ret |= 1 << 3;
        }
        if item.timer_overflow {
            ret |= 1 << 2;
        }
        if item.lcdc {
            ret |= 1 << 1;
        }
        if item.vblank {
            ret |= 1;
        }
        return ret;
    }
}

#[derive(Copy, Clone)]
pub struct Joypad {
    pub select_action: bool,
    pub select_direction: bool,
    pub p13_in: bool,
    pub p12_in: bool,
    pub p11_in: bool,
    pub p10_in: bool,
}

impl From<Joypad> for u8 {
    fn from(item: Joypad) -> Self {
        let mut ret: u8 = 0;

        if item.select_action {
            ret |= 1 << 5;
        }
        if item.select_direction {
            ret |= 1 << 4;
        }
        if item.p13_in {
            ret |= 1 << 3;
        }
        if item.p12_in {
            ret |= 1 << 2;
        }
        if item.p11_in {
            ret |= 1 << 1;
        }
        if item.p10_in {
            ret |= 1;
        }
        return ret;
    }
}

impl From<u8> for Joypad {
    fn from(item: u8) -> Self {
        Joypad {
            select_action: item & (1 << 5) != 0,
            select_direction: item & (1 << 4) != 0,
            p13_in: item & (1 << 3) != 0,
            p12_in: item & (1 << 2) != 0,
            p11_in: item & (1 << 1) != 0,
            p10_in: item & (1 << 0) != 0,
        }
    }
}

pub struct Memory {
    // Memory bounds
    pub rom_address_bound: Range<usize>,
    pub vram_address_bound: Range<usize>,
    pub sram8_address_bound: Range<usize>,
    pub wram_address_bound: Range<usize>,
    pub iram8_echo_address_bound: Range<usize>,
    pub oam_address_bound: Range<usize>,
    pub io_address_bound: Range<usize>,
    pub hram_address_bound: Range<usize>,

    // Memory
    boot: &'static [u8; 256],
    pub rom: Box<dyn MemoryBankController>,
    vram: [u8; 0x2000],
    hram: [u8; 0x7F],
    wram: [u8; 0x2000],
    oam: [u8; 0x100],

    // TODO: Sort this, maybe export ?
    // Special Registers
    pub boot_rom_disable: u8,
    pub ei: Ei,
    pub iflag: Ei,

    pub jpad: Joypad,
    //   Sound
    nr11: u8,
    //   Graphics
    pub ly: u8,
    pub scx: u8,
    pub scy: u8,
    pub lcdc: Lcdc,
    pub stat: Stat,
    pub obp0: u8,
    pub obp1: u8,
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
            wram_address_bound: Range {
                start: 0xC000,
                end: 0xE000,
            },
            iram8_echo_address_bound: Range {
                start: 0xE000,
                end: 0xFE00,
            },
            oam_address_bound: Range {
                start: 0xFE00,
                end: 0xFF00,
            },
            io_address_bound: Range {
                start: 0xFF00,
                end: 0xFF80,
            },
            hram_address_bound: Range {
                start: 0xFF80,
                end: 0xFFFF,
            },

            // Memory
            boot: include_bytes!("../../res/boot.bin"),
            rom: Box::new(MBC0::new([].to_vec())), // By default we "load" a MBC0
            vram: [0; 0x2000],
            hram: [0; 0x7F],
            wram: [0; 0x2000],
            oam: [0; 0x100],

            // Special registers
            boot_rom_disable: 0,
            nr11: 0,
            ly: 0x0, // TODO: This should be 0 on startup, this is set so that the boot sequence doesn't loop forever
            scy: 0,
            scx: 0,
            jpad: Joypad::from(0xFF),
            ei: Ei::from(0x0),
            iflag: Ei::from(0x0),
            lcdc: Lcdc::from(0x91),
            stat: Stat::from(0x0),
            obp0: 0xFF,
            obp1: 0xFF,
        }
    }

    pub fn write_io_u8(&mut self, value: u8, address: usize) {
        match address {
            0xFF01 => (), // TODO Implement SB
            0xFF02 => (), // TODO Implement SC
            0xFF11 => self.nr11 = value,
            0xFF12 => (), // TODO Implement NR12
            0xFF13 => (), // TODO Implement NR13
            0xFF14 => (), // TODO Implement NR13
            0xFF24 => (), // TODO Implement NR50
            0xFF25 => (), // TODO Implement NR51
            0xFF26 => (), // TODO Implement NR52
            0xFF40 => self.lcdc = Lcdc::from(value),
            0xFF41 => self.stat = Stat::from(value),
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => self.ly = value,
            0xFF47 => (),                // TODO Implement BGP
            0xFF48 => self.obp0 = value, // TODO Implement BGP
            0xFF49 => self.obp1 = value, // TODO Implement BGP
            0xFF0F => self.iflag = Ei::from(value),
            0xFF50 => self.boot_rom_disable = value, // This is a special register used by the boot rom
            0xFFFF => self.ei = Ei::from(value),
            _ => (), // panic!("Unknown IO register W: {:#02x}", address)
        }
    }

    fn read_io_u8(&self, address: usize) -> u8 {
        match address {
            0xFF00 => u8::from(self.jpad),
            0xFF01 => 0, // TODO Implement SB
            0xFF02 => 0, // TODO Implement SC
            0xFF11 => self.nr11,
            0xFF12 => 0, // TODO Implement NR12
            0xFF13 => 0, // TODO Implement NR13
            0xFF14 => 0, // TODO Implement NR14
            0xFF24 => 0, // TODO Implement NR50
            0xFF25 => 0, // TODO Implement NR51
            0xFF26 => 0, // TODO Implement NR52
            0xFF40 => u8::from(self.lcdc),
            0xFF41 => u8::from(self.stat),
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF47 => 0xFC, // TODO Implement it and link it to selected cpalette
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,

            0xFF0F => u8::from(self.iflag),
            // This is a special register used by the boot rom
            0xFF50 => self.boot_rom_disable,
            0xFFFF => u8::from(self.ei),
            _ => panic!("Unknown IO register R: {:#02x}", address),
        }
    }

    pub fn write_u8(&mut self, value: u8, address: usize) {
        if self.rom_address_bound.contains(&address) {
            self.rom.write_u8(address, value);
        } else if self.hram_address_bound.contains(&address) {
            self.hram[address - self.hram_address_bound.start] = value;
        } else if self.vram_address_bound.contains(&address) {
            self.vram[address - self.vram_address_bound.start] = value;
        } else if self.wram_address_bound.contains(&address) {
            self.wram[address - self.wram_address_bound.start] = value;
        } else if self.oam_address_bound.contains(&address) {
            self.oam[address - self.oam_address_bound.start] = value;
        } else if address == 0xFF50 || address == 0xFFFF || self.io_address_bound.contains(&address)
        {
            self.write_io_u8(value, address);
        } else {
            panic!(
                "Write: {:#02x} is not an implemented memory address",
                address
            );
        }
    }

    pub fn read_u8(&self, address: usize) -> u8 {
        if self.rom_address_bound.contains(&address) {
            if self.boot_rom_disable == 0 && address <= 0xFF {
                // During boot, any read from value 0x0 to 0xFF is redirected to the boot rom
                return self.boot[address];
            }
            return self.rom.read_u8(address);
        } else if self.hram_address_bound.contains(&address) {
            return self.hram[address - self.hram_address_bound.start];
        } else if self.vram_address_bound.contains(&address) {
            return self.vram[address - self.vram_address_bound.start];
        } else if self.wram_address_bound.contains(&address) {
            return self.wram[address - self.wram_address_bound.start];
        } else if self.oam_address_bound.contains(&address) {
            return self.oam[address - self.oam_address_bound.start];
        } else if self.io_address_bound.contains(&address) || address == 0xFFFF {
            return self.read_io_u8(address);
        } else {
            panic!(
                "Read: {:#02x} is not an implemented memory address",
                address
            );
        }
    }
}
