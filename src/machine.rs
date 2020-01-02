use crate::cpu::Cpu;
use crate::ram::Ram;

//#[derive(Debug)]
pub struct Machine {
    cpu: Cpu,
    ram: Ram
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            cpu: Cpu::new(),
            ram: Ram::new(),
        }
    }

    pub fn load_rom(&mut self, data: Vec<u8>) { //TODO return result
        self.ram.load_rom(data);
    }

    pub fn start(&mut self) { //TODO return result
        unimplemented!();
    }
}