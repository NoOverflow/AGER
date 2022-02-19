mod gameboy;

use gameboy::Gameboy;

fn main() {
    let gb: Gameboy = Gameboy::new();

    gb.dump_bios();
}
