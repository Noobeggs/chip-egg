mod display;
mod options;
use display::Display;
use options::Options;

pub struct Chip8 {
    pc: u16,
    ir: u16,
    vr: [u16; 16],
    stack: [u16; 16],
    memory: [u8; 4096],
    display: Display,
    delay_timer: u8,
    sound_timer: u8,
    options: Options
}

impl Chip8 {
    pub fn new() -> Chip8 {
        todo!();
    }

    pub fn fetch(&mut self) -> u16 {
        // What SHOULD happen when we read the end of memory???
        // wrap back to the beginning???
        let opcode = (u16::from(self.memory[self.pc as usize]) << 8)
            | u16::from(self.memory[self.pc.wrapping_add(1) as usize]);
        self.pc = self.pc.wrapping_add(2);
        opcode
    }

    pub fn decode() {

    }

    pub fn execute() {

    }
}