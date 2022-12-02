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
    sp: usize,
    stack: [u16; 16],
    memory: [u8; RAM],
    display: Display,
    delay_timer: u8,
    sound_timer: u8,
    options: Options,
    last_tick: Instant,
    keyboard: [bool; 16],
}

impl Chip8 {
    pub fn new(options: Options) -> Chip8 {
        // todo!();
        let mut memory = [0; RAM];

        let font = options.font();

        memory[0x50 .. (0x50 + font.len())].clone_from_slice(&font);
        Chip8 {
            pc: 0x200,
            ir: 0,
            vr: [0; 16],
            sp: 0,
            stack: [0; 16],
            memory: memory,
            display: Display::new(),
            delay_timer: 0,
            sound_timer: 0,
            options: options,
            last_tick: Instant::now(),
            keyboard: [false; 16],
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

    pub fn decode(&mut self, opcode: u16) -> Result<(), String> {
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
            (0x0, 0x0, 0xE, 0xE) => {
                if self.sp == 0 {
                    return Err(String::from("Attempted to pop from empty stack."));
                }
                self.pc = self.stack[self.sp];
                self.sp -= 1;
            }
            (0x1, _, _, _) => self.pc = nnn,
            (0x2, _, _, _) => {
                self.sp += 1;
                if self.sp >= self.stack.len() {
                    return Err(String::from("Stack limit reached."));
                }
                self.stack[self.sp] = self.pc;
                self.pc = nnn;
            }
            (0x3, _, _, _) => {
                if self.vr[x] == nn {
                    self.fetch();
                }
            }
            (0x4, _, _, _) => {
                if self.vr[x] != nn {
                    self.fetch();
                }
            }
            (0x5, _, _, 0x0) => {
                if self.vr[x] == self.vr[y] {
                    self.fetch();
                }
            }
            (0x6, _, _, _) => self.vr[x] = nn,
            (0x7, _, _, _) => self.vr[x] = self.vr[x].wrapping_add(nn),
            // Logical and Arithmetic Instructions
            (0x8, _, _, 0x0) => self.vr[x] = self.vr[y],
            (0x8, _, _, 0x1) => self.vr[x] |= self.vr[y],
            (0x8, _, _, 0x2) => self.vr[x] &= self.vr[y],
            (0x8, _, _, 0x3) => self.vr[x] ^= self.vr[y],
            (0x8, _, _, 0x4) => {
                let (result, carry) = self.vr[x].overflowing_add(self.vr[y]);
                self.vr[x] = result;
                self.vr[0xF] = if carry {1} else {0};
            }
            (0x8, _, _, 0x5) => {
                let (result, carry) = self.vr[x].overflowing_sub(self.vr[y]);
                self.vr[x] = result;
                self.vr[0xF] = if carry {0} else {1}; // VF set to 0 if underflow.
            }
            (0x8, _, _, 0x6) => {
                // Ambiguous instruction! TODO: option to switch implementations.
                self.vr[x] = self.vr[y];
                self.vr[0xF] = self.vr[x] & 1;
                self.vr[x] = self.vr[x] >> 1;
            }
            (0x8, _, _, 0x7) => {
                let (result, carry) = self.vr[y].overflowing_sub(self.vr[x]);
                self.vr[x] = result;
                self.vr[0xF] = if carry {0} else {1};
            }
            (0x8, _, _, 0xE) => {
                // Ambiguous instruction! TODO: option to switch implementations.
                self.vr[x] = self.vr[y];
                self.vr[0xF] = (self.vr[x] & 0x80) >> 7;
                self.vr[x] = self.vr[x] << 1;
            }
            (0x9, _, _, 0x0) => {
                if self.vr[x] != self.vr[y] {
                    self.fetch();
                }
            }
            (0xA, _, _, _) => self.ir = nnn,
            (0xB, _, _, _) => {
                // Ambiguous instruction! TODO: option to switch implementations.
                self.pc = nnn + u16::from(self.vr[0]);
            }
            (0xC, _, _, _) => self.vr[x] = fastrand::u8(..) & nn,
            (0xD, _, _, _) => {
                // let mut sprite = Vec::<Vec<u8>>::new();
                // for h in 0..n {
                //     let mut row = Vec::<u8>::new();
                //     let mut byte = 
                // }
                let sprite = &self.memory[self.ir as usize .. (self.ir + n) as usize];
                self.display.draw(sprite, self.vr[x], self.vr[y]);
            }
            (0xE, _, 0x9, 0xE) => if self.keyboard[self.vr[x] as usize] {self.fetch();},
            (0xE, _, 0xA, 0x1) => if !self.keyboard[self.vr[x] as usize] {self.fetch();},
            (0xF, _, 0x0, 0x7) => self.vr[x] = self.delay_timer,
            (0xF, _, 0x0, 0xA) => {
                self.pc = self.pc.wrapping_sub(2);
                for key in 0..self.keyboard.len() {
                    if self.keyboard[key] {
                        self.vr[x] = key as u8;
                        self.fetch();
                        self.keyboard[key] = false;
                        break;
                    }
                }
            }
            (0xF, _, 0x1, 0x5) => self.delay_timer = self.vr[x],
            (0xF, _, 0x1, 0x8) => self.sound_timer = self.vr[x],
            (0xF, _, 0x1, 0xE) => {
                // Ambiguous instruction! TODO: option to switch implementations.
                if self.ir < 0x0FFF && (self.ir + u16::from(self.vr[x])) >= 0x1000 {
                    self.vr[0xF] = 1
                }
                self.ir = self.ir.wrapping_add(u16::from(self.vr[x]));
            }
            (0xF, _, 0x2, 0x9) => self.ir = 0x50 + u16::from((self.vr[x] & 0x0F) * 5),
            (0xF, _, 0x3, 0x3) => {
                self.memory[self.ir as usize] = self.vr[x] / 100;
                self.memory[self.ir as usize + 1] = (self.vr[x] / 10) % 10;
                self.memory[self.ir as usize + 2] = self.vr[x] % 10;
            }
            (0xF, _, 0x5, 0x5) => {
                // Ambiguous instruction! TODO: option to switch implementations.
                for i in 0..self.vr.len() {
                    self.memory[self.ir as usize + i] = self.vr[i];
                }
            }
            (0xF, _, 0x6, 0x5) => {
                // Ambiguous instruction! TODO: option to switch implementations.
                for i in 0..self.vr.len() {
                    self.vr[i] = self.memory[self.ir as usize + i];
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn execute() {
        todo!();
    }

    pub fn run_cpu_cycle(&mut self) -> Result<(), String> {
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
        self.decode(opcode)?;
        Ok(())
    }
}