use crate::ram::Ram;


//#[derive(Debug)]
pub struct Bus {
   ram: Ram,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: Ram::new(),
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

}