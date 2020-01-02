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

}