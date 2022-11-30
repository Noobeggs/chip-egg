pub struct Display {
    pub display: [[u8; 32]; 64],
    pub width: u8,
    pub height: u8,
}

impl Display {
    pub fn new() -> Display {
        Display {
            display: [[0; 32]; 64],
            width: 64,
            height: 32,
        }
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
            for pixel in 0..8 {
                if sprite_row & (0x80 >> pixel) != 0 {
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