const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    display: [[u8; HEIGHT]; WIDTH],
    // Probably don't need these... but might need it later if we want to support other CHIP-8 variants.
    width: u8,
    height: u8,
    // for redrawing if the display buffer changes.
    redraw: bool,
}

impl Display {
    pub fn new() -> Display {
        Display {
            display: [[0; HEIGHT]; WIDTH],
            width: WIDTH as u8,
            height: HEIGHT as u8,
            redraw: true,
        }
    }

    pub fn display(&self) -> [[u8; HEIGHT]; WIDTH] {
        self.display
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn set_width(&mut self, new_width: u8) {
        self.width = new_width;
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn set_height(&mut self, new_height: u8) {
        self.height = new_height;
    }

    pub fn redraw(&self) -> bool {
        self.redraw
    }

    pub fn reset_redraw(&mut self) {
        self.redraw = false;
    }

    pub fn clear_screen(&mut self) {
        self.display = [[0u8; HEIGHT]; WIDTH];
    }

    pub fn draw(&mut self, sprite: &[u8], vx: u8, vy: u8) -> u8 {
        let x = vx & (self.width-1);
        let y = vy & (self.height-1);
        let mut collision = 0;

        // for (row, sprite_row) in sprite.iter().enumerate() {
        //     if row + y as usize >= self.height as usize {
        //         break;
        //     }
        //     for (col, sprite_pixel) in sprite_row.iter().enumerate() {
        //         if (col + x) as usize >= self.width as usize {
        //             break;
        //         }
        //         if sprite_pixel == 1 {
        //             if collision == 0 && display[x][y] == 1 {
        //                 collision = 1
        //             }
        //             display[x][y] ^= 1
        //         }
        //     }
        // }

        for (row, sprite_row) in sprite.iter().enumerate() {
            if row + y as usize >= self.height as usize {
                break;
            }
            for pixel in 0..7 {
                if x + pixel as u8 >= self.width {
                    break;
                }
                if sprite_row & (0x80 >> pixel) != 0 {
                    if !self.redraw {
                        self.redraw = true;
                    }
                    if collision == 0 && self.display[x as usize + pixel][y as usize + row] == 1 {
                        collision = 1
                    }
                    self.display[x as usize + pixel][y as usize + row] ^= 1
                }
            }
        }

        collision
    }
}