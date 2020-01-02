use crate::cpu::Cpu;
use crate::bus::Bus;

//#[derive(Debug)]
pub struct Machine {
    cpu: Cpu,
    bus: Bus
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            cpu: Cpu::new(),
            bus: Bus::new(),
        }
    }

    pub fn load_rom(&mut self, data: Vec<u8>) { //TODO return result
        self.bus.load_rom(data);
    }

    pub fn start(&mut self) { //TODO return result
        loop {
            self.cpu.execute_instruction(&self.bus);
        }
    }
}