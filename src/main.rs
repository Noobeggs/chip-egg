mod window;

use chip_egg::Chip8;
use chip_egg::Options;
use window::run;

use std::env;
use std::fs::read;

const CPU_CLOCK: u64 = 1428; // 700Hz

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].to_owned();

    // let file = File::open(filename).expect("Error opening file.");
    // let mut buf = [0u8; 3584];
    let rom = read(filename).expect("Error reading file.");

    pollster::block_on(run(rom));
}