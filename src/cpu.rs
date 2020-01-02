use crate::bus::Bus;

//#[derive(Debug)]
pub struct Cpu {
    vx: [u8; 16],
    i: u16,
    delay_timer_register: u8,
    sound_timer_register: u8,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 16],
    //TODO add bus reference as a property, read about lifetimes
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            i: 0,
            delay_timer_register: 0,
            sound_timer_register: 0,
            program_counter: 0,
            stack_pointer: 0,
            stack: [0;16],
        }
    }

    pub fn execute_instruction(&mut self, bus: &Bus) { 
        unimplemented!();
    }
}