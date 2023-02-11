mod gameboy;
mod window;

use gameboy::Gameboy;
use spin_sleep::LoopHelper;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn clock_loop(gb: Arc<Mutex<Gameboy>>, tx: Sender<Vec<u32>>) {
    let mut loop_helper = LoopHelper::builder()
        .report_interval_s(0.5)
        .build_with_target_rate(60);
    let mut stop = false;

    gb.lock()
        .unwrap()
        .load_cartridge("res/test/individual/02-interrupts.gb");
    gb.lock().unwrap().power_up();

    while !stop {
        let delta = loop_helper.loop_start();

        gb.lock().unwrap().cycle(delta.as_secs());
        if let Some(_fps) = loop_helper.report_rate() {
            // println!("Current FPS: {}", fps);
        }
        let buffer = gb.lock().unwrap().get_screen_buffer();

        match tx.send(buffer) {
            Ok(_) => (),
            Err(_) => stop = true,
        }
        loop_helper.loop_sleep();
    }
}

fn main() {
    let (tx, rx): (Sender<Vec<u32>>, Receiver<Vec<u32>>) = channel();
    let gb: Arc<Mutex<Gameboy>> = Arc::new(Mutex::new(Gameboy::new()));

    {
        let gb_clone = gb.clone();

        thread::spawn(move || {
            clock_loop(gb_clone, tx);
        });
    }

    {
        let gb_clone = gb.clone();
        let rx_mutex = Arc::new(Mutex::from(rx));

        window::Window::new().init_window(gb_clone, rx_mutex);
    }
}
