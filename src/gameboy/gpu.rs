use super::memory::Memory;

pub struct Lcdc {
    pub lcd_control_op: bool,
    pub window_tmap_displ_select: bool,
    pub window_display: bool,
    pub bg_win_tile_data_select: bool,
    pub bg_tmap_display_select: bool,
    pub sprite_size: bool,
    pub sprite_display: bool,
    pub bg_win_display: bool,
}

impl From<u8> for Lcdc {
    fn from(item: u8) -> Self {
        Lcdc {
            lcd_control_op: item & (1 << 7) != 0,
            window_tmap_displ_select: item & (1 << 6) != 0,
            window_display: item & (1 << 5) != 0,
            bg_win_tile_data_select: item & (1 << 4) != 0,
            bg_tmap_display_select: item & (1 << 3) != 0,
            sprite_size: item & (1 << 2) != 0,
            sprite_display: item & (1 << 1) != 0,
            bg_win_display: item & (1 << 0) != 0,
        }
    }
}

pub struct Gpu {
    buffer: Vec<u32>,
}

const IBUFFER_SIZE: usize = 256;

impl Gpu {
    pub fn new() -> Self {
        Gpu {
            buffer: vec![0; IBUFFER_SIZE * IBUFFER_SIZE],
        }
    }

    fn get_sprite_data(&self, mut mem: &Memory, index: usize) -> [[u8; 8]; 8] {
        let tdata_address_range = if mem.lcdc.bg_win_tile_data_select {
            0x8000..0x9000
        } else {
            0x8800..0x9800
        };
        let mut i: usize = 0;
        let mut sprite_data = [[0u8; 8]; 8];

        while i < 16 {
            let vl: u8 = mem.read_u8(tdata_address_range.start + (index * 16) + i);
            let vh: u8 = mem.read_u8(tdata_address_range.start + (index * 16) + i + 1);
            let y: usize = i / 2;

            for j in 0..8_isize {
                let v = ((vl >> (7 - j)) & 0x1) | if vh & (1 << 7 - j) != 0 { 2 } else { 0 };

                sprite_data[y][j as usize] = v;
            }
            i += 2
        }
        return sprite_data;
    }

    fn gbcolor_to_rgb(&self, gb_color: u8) -> u32 {
        match gb_color {
            0x0 => 0x0f380f,
            0x1 => 0x306230,
            0x2 => 0x8bac0f,
            0x3 => 0x9bbc0f,
            _ => panic!("Impossible color value"),
        }
    }

    fn draw_background(&mut self, mut mem: &Memory, buffer: &mut Vec<u32>) {
        let bg_map_range = if mem.lcdc.bg_tmap_display_select {
            0x9C00..0xA000
        } else {
            0x9800..0x9C00
        };

        for tile_y in 0..32 {
            for tile_x in 0..32 {
                // TODO Add signed IDs support
                let tid = mem.read_u8(bg_map_range.start + (tile_y * 32) + tile_x);
                let tdata = self.get_sprite_data(mem, tid as usize);

                for ly in 0..8 {
                    for lx in 0..8 {
                        let mut by = mem.scy as usize + tile_y * 8 + ly;
                        let bx = tile_x * 8 + lx;

                        if by > 255 {
                            by = by - 255;
                        }
                        buffer[by * 256 + bx] = self.gbcolor_to_rgb(tdata[ly][lx]);
                    }
                }
            }
        }
    }

    pub fn cycle(&mut self, mut mem: &Memory, buffer: &mut Vec<u32>) {
        if mem.lcdc.bg_win_display {
            self.draw_background(mem, buffer);
            if mem.lcdc.window_display {}
        }
    }
}
