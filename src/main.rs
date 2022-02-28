extern crate minifb;
mod gameboy;

use gameboy::Gameboy;
use minifb::{Key, Window, WindowOptions};

const WINDOW_WIDTH: usize = 160 * 4;
const WINDOW_HEIGHT: usize = 144 * 4;

fn main() {
    let gb: &mut Gameboy = &mut Gameboy::new();
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "AGER",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    gb.load_cartridge("res/test/licensed/tetris.gb");
    gb.power_up();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
        gb.cycle();
    }
}
