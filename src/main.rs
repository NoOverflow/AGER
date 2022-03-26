extern crate minifb;
mod gameboy;

use gameboy::Gameboy;
use minifb::{Key, Window, WindowOptions};
use spin_sleep::LoopHelper;

const SCALE: usize = 4;
const WINDOW_WIDTH: usize = 256; // 160 * SCALE;
const WINDOW_HEIGHT: usize = 256; // 144 * SCALE;

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

    let mut loop_helper = LoopHelper::builder()
        .report_interval_s(0.5) // report every half a second
        .build_with_target_rate(60.0); // limit to 250 FPS if possible

    while !gb.stop {
        let mut delta = loop_helper.loop_start();

        gb.cycle(&mut buffer, delta.as_secs());
        if let Some(fps) = loop_helper.report_rate() {
            println!("Current FPS: {}", fps);
        }
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
        loop_helper.loop_sleep();
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}
