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

pub struct Stat {
    pub coincidence_select: bool,
    pub select_mode_10: bool,
    pub select_mode_01: bool,
    pub select_mode_00: bool,
    pub coincidence_flag: bool,
    pub mode_flag: u8,
}

impl From<u8> for Stat {
    fn from(item: u8) -> Self {
        Stat {
            coincidence_select: item & (1 << 6) != 0,
            select_mode_10: item & (1 << 5) != 0,
            select_mode_01: item & (1 << 4) != 0,
            select_mode_00: item & (1 << 3) != 0,
            coincidence_flag: item & (1 << 2) != 0,
            mode_flag: item & 0x3,
        }
    }
}

pub struct Gpu {
    buffer: Vec<u32>,
    pub mode_clock: usize,
    pub vblank: bool,
}

const IBUFFER_SIZE: usize = 256;

impl Gpu {
    pub fn new() -> Self {
        Gpu {
            buffer: vec![0xFF1B0F0F; IBUFFER_SIZE * IBUFFER_SIZE],
            mode_clock: 0,
            vblank: false,
        }
    }

    fn get_sprite_data(&self, mem: &Memory, index: usize) -> [[u8; 8]; 8] {
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

    pub fn get_screen_buffer(&self, mem: &Memory) -> Vec<u32> {
        let mut ret: Vec<u32> = vec![0x0; 160 * 144];

        for y in 0..144 {
            let mut buffer_y = y + mem.scy as isize;

            if buffer_y > 255 {
                buffer_y = buffer_y - 255;
            } else if buffer_y < 0 {
                buffer_y = 255 + buffer_y;
            }
            for x in 0..160 {
                ret[y as usize * 160 + x] = self.buffer[buffer_y as usize * 256 + x];
            }
        }
        return ret;
    }

    fn gbcolor_to_rgb(&self, gb_color: u8) -> u32 {
        match gb_color {
            // ABGR
            0x0 => 0xFF1B0F0F,
            0x1 => 0xFF755A56,
            0x2 => 0xFFBEB7C6,
            0x3 => 0xFFF6FBFA,
            _ => panic!("Impossible color value"),
        }
    }

    fn draw_background_line(&mut self, mem: &mut Memory) {
        let bg_map_range = if mem.lcdc.bg_tmap_display_select {
            0x9C00..0xA000
        } else {
            0x9800..0x9C00
        };
        let tile_y: usize = mem.ly as usize / 8;

        for tile_x in 0..32 {
            // TODO Add signed IDs support
            let tid = mem.read_u8(bg_map_range.start + (tile_y * 32) + tile_x);
            let tdata = self.get_sprite_data(mem, tid as usize);
            let ly: usize = mem.ly as usize - (tile_y * 8);

            for lx in 0..8 {
                let mut by = tile_y as isize * 8 + ly as isize;
                let bx = tile_x * 8 + lx;

                if by > 255 {
                    by = by - 255;
                } else if by < 0 {
                    by = 255 + by;
                }
                self.buffer[by as usize * 256 + bx] = self.gbcolor_to_rgb(tdata[ly][lx]);
            }
        }
    }

    fn draw_line(&mut self, mem: &mut Memory) {
        if mem.lcdc.bg_win_display {
            self.draw_background_line(mem);
            if mem.lcdc.window_display {}
        }
    }

    pub fn cycle(&mut self, mem: &mut Memory, ticks: usize) -> bool {
        if !mem.lcdc.lcd_control_op {
            false;
        }

        // Timings taken from page 54 gameboy CPU man
        self.mode_clock += ticks;
        if self.mode_clock > 456 {
            mem.ly = (mem.ly + 1) % 154;
            self.mode_clock = 0;
        }
        if mem.ly < 144 {
            if self.mode_clock <= 80 {
                if mem.stat.mode_flag != 0b10 {
                    mem.stat.mode_flag = 0b10;
                    self.draw_line(mem);
                    return true;
                }
            } else if self.mode_clock <= 80 + 172 {
                mem.stat.mode_flag = 0b11;
            } else {
                mem.stat.mode_flag = 0b00;
            }
        } else {
            if mem.stat.mode_flag != 0b01 {
                mem.stat.mode_flag = 0b01;
                mem.iflag.vblank = true;
            }
            self.vblank = true
        }
        return false;
    }
}
