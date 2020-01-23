use crate::ram::Ram;
use crate::display::Display;
use crate::keyboard::Keyboard;
use ggez::{Context, ContextBuilder, GameResult};


//#[derive(Debug)]
pub struct Bus {
   ram: Ram,
   keyboard: Keyboard,
   display: Display,
   //TODO put a context reference with lifetimes

}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: Ram::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
        }
    }

    pub fn load_rom(&mut self, data: Vec<u8>) { //TODO return result
        self.ram.load_rom(data);
    }

    pub fn write_byte(&mut self, address: u16, data: u8) {
        self.ram.write_byte(address, data);
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.ram.read_byte(address)
    }

    pub fn set_last_key_pressed() {

    }

    pub fn get_last_key_pressed () {

    }

}