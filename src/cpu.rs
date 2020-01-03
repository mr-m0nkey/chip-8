use crate::bus::Bus;
use crate::ram::PROGRAM_START;

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
            program_counter: PROGRAM_START,
            stack_pointer: 0,
            stack: [0;16],
        }
    }

    pub fn execute_instruction(&mut self, bus: &Bus) { 
        let most_significant_byte = bus.read_byte(self.program_counter) as u16;
        let least_significant_byte = bus.read_byte(self.program_counter + 1) as u16;
        let instruction: u16 = (most_significant_byte << 8) | least_significant_byte;

        let nnn = instruction & 0x0FFF;
        let n =  (instruction & 0xF) as u8;
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;
        let kk = (instruction & 0xFF) as u8;

        match (instruction & 0xF000) >> 12 {
            0x0 => {
                match kk {
                    0xE0 => {
                        //TODO clear the display
                    }
                    0xEE => {
                        self.program_counter = self.stack[self.stack_pointer as usize];
                        self.stack[self.stack_pointer as usize] = 0;
                        self.stack_pointer -= 1;
                    }
                    _ => {
                        panic!("Unhandled instruction at 0x0 for {:#X}", kk);
                    }
                }
             
            }

            0x1 => {
                self.program_counter = nnn;
            }

            0x2 => {
                //TODO Call subroutine at nnn.
                // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
            }

            0x3 => {
                //TODO Skip next instruction if Vx = kk.
                // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
            }

            _ => {
                panic!("Unhandled instruction");
            }

        }


    }
}