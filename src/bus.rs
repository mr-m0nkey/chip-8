use crate::display::Display;
use crate::ram::Ram;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

//#[derive(Debug)]
pub struct Bus {
    ram: Ram,
    pub display: Display,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: Ram::new(),
            display: Display::new(),
        }
    }

    pub fn load_rom(&mut self, data: Vec<u8>) {
        //TODO return result
        self.ram.load_rom(data);
    }

    pub fn write_byte(&mut self, address: u16, data: u8) {
        self.ram.write_byte(address, data);
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.ram.read_byte(address)
    }

    pub fn set_last_key_pressed() {}

    pub fn get_last_key_pressed() {}

    pub fn draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        //TODO move implementation to Display.rs
        let mut x_coord = x % WIDTH as u8;
        let y_coord = y % HEIGHT as u8;
        let mut erased = false;
        let mut byte = byte;
        for _ in 0..8 {
            let bit = (byte & 0b1000_0000) >> 7;
            if x_coord as u16 * y_coord as u16 > 2048 {
                panic!("{} and {}", x_coord, y_coord);
            }
            let index = self.get_screen_index(x_coord, y_coord);
            if index >= 2048 {
                panic!("{} and {}", x_coord, y_coord);
            }
            let previous_value = self.display.screen[index as usize];
            self.display.screen[index] ^= bit;
            if previous_value == 0x1 && self.display.screen[index as usize] == 0x0 {
                erased = true;
            }

            byte <<= 1;
            x_coord += 1;
        }

        erased
    }

    pub fn get_screen_index(&self, x: u8, y: u8) -> usize {
        Display::get_index_from_coords(x as usize, y as usize) as usize
    }
}
