mod gameboy;

use gameboy::Gameboy;

fn main() {
    let gb: &mut Gameboy = &mut Gameboy::new();

    gb.power_up();
    loop {
        gb.cycle();
    }
}
