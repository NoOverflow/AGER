mod gameboy;

use gameboy::Gameboy;

fn main() {
    let gb: &mut Gameboy = &mut Gameboy::new();

    gb.load_cartridge("res/test/licensed/tetris.gb");
    gb.power_up();
    loop {
        gb.cycle();
    }
}
