mod display;
mod options;

pub use display::Display;
pub use options::Options;

use std::time::{Duration, Instant};

const RAM: usize = 4096;
const TIMER_RATE: u64 = 16666; //60Hz

pub struct Chip8 {
    pc: u16,
    ir: u16,
    vr: [u8; 16],
    stack: [u16; 16],
    memory: [u8; RAM],
    display: Display,
    delay_timer: u8,
    sound_timer: u8,
    options: Options,
    last_tick: Instant,
}

impl Chip8 {
    pub fn new(options: Options) -> Chip8 {
        // todo!();
        let mut memory = [0; RAM];

        let font = options.font();

        memory[0x50 .. (0x50 + font.len())].clone_from_slice(&font);
        Chip8 {
            pc: 0,
            ir: 0,
            vr: [0; 16],
            stack: [0; 16],
            memory: memory,
            display: Display::new(),
            delay_timer: 0,
            sound_timer: 0,
            options: options,
            last_tick: Instant::now(),
        }
    }

    pub fn display(&self) -> &Display {
        &self.display
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.memory[0x200..][..rom.len()].copy_from_slice(rom.as_slice());
    }

    pub fn fetch(&mut self) -> u16 {
        // What SHOULD happen when we read the end of memory???
        // wrap back to the beginning???
        let opcode = (u16::from(self.memory[self.pc as usize]) << 8)
            | u16::from(self.memory[self.pc.wrapping_add(1) as usize]);
        self.pc = self.pc.wrapping_add(2);
        opcode
    }

    pub fn decode(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = opcode & 0x000F;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        let nib_1 = (opcode & 0xF000) >> 12;
        let nib_2 = (opcode & 0x0F00) >> 8;
        let nib_3 = (opcode & 0x00F0) >> 4;
        let nib_4 = opcode & 0x000F;

        match (nib_1, nib_2, nib_3, nib_4) {
            (0x0, 0x0, 0xE, 0x0) => self.display.clear_screen(),
            (0x1, _, _, _) => self.pc = nnn,
            (0x6, _, _, _) => self.vr[x] = nn,
            (0x7, _, _, _) => self.vr[x] = self.vr[x].wrapping_add(nn),
            (0xA, _, _, _) => self.ir = nnn,
            (0xD, _, _, _) => {
                // let mut sprite = Vec::<Vec<u8>>::new();
                // for h in 0..n {
                //     let mut row = Vec::<u8>::new();
                //     let mut byte = 
                // }
                let sprite = &self.memory[self.ir as usize .. (self.ir + n) as usize];
                self.display.draw(sprite, self.vr[x], self.vr[y]);
            }
            _ => {}
        }
    }

    pub fn execute() {
        todo!();
    }

    pub fn run_cpu_cycle(&mut self) {
        self.display.reset_redraw();
        if self.last_tick.elapsed() >= Duration::from_micros(TIMER_RATE) {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }
        }

        let opcode = self.fetch();
        self.decode(opcode);
    }
}