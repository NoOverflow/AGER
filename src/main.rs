mod gameboy;
mod window;

use gameboy::Gameboy;
use spin_sleep::LoopHelper;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

fn clock_loop(tx: Sender<Vec<u32>>) {
    let gb: &mut Gameboy = &mut Gameboy::new();
    let mut loop_helper = LoopHelper::builder()
        .report_interval_s(0.5)
        .build_with_target_rate(60.0);
    let mut stop = false;

    gb.load_cartridge("res/test/licensed/tetris.gb");
    gb.power_up();
    while !stop {
        let delta = loop_helper.loop_start();

        gb.cycle(delta.as_secs());
        if let Some(fps) = loop_helper.report_rate() {
            println!("Current FPS: {}", fps);
        }
        let buffer = gb.get_screen_buffer();

        match tx.send(buffer) {
            Ok(_) => (),
            Err(_) => stop = true,
        }
        loop_helper.loop_sleep();
    }
}

fn main() {
    let (tx, rx): (Sender<Vec<u32>>, Receiver<Vec<u32>>) = channel();

    thread::spawn(move || {
        clock_loop(tx);
    });
    window::init_window(rx);
}
