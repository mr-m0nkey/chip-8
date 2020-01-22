use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::keyboard::Keyboard;
use crate::display::Display;


//#[derive(Debug)]
pub struct Machine {
    cpu: Cpu,
    bus: Bus,
    keyboard: Keyboard,
    display: Display,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            cpu: Cpu::new(),
            bus: Bus::new(),
            keyboard: Keyboard::new(),
            display: Display::new()
        }
    }

    pub fn load_rom(&mut self, data: Vec<u8>) { //TODO return result
        self.bus.load_rom(data);
    }

    pub fn start(&mut self) { //TODO return result
        loop {

            //TODO check for keyboard input
            if self.cpu.should_execute {
                self.cpu.execute_instruction(&mut self.bus);
            }

            //on keypress event, check if cpu is waiting for keypress
            //if it is set the last key pressed, if not do nothing
            
            
        }
    }
}