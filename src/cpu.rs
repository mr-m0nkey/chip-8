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
        println!("Current executing instrcution: {:#X}", instruction);
        match (instruction & 0xF000) >> 12 {
            0x0 => {
                match kk {
                    0xE0 => {
                        //TODO clear the display
                        self.program_counter += 2;
                    }
                    0xEE => {
                        self.program_counter = self.stack[self.stack_pointer as usize];
                        self.stack_pointer -= 1;
                    }
                    _ => {
                        panic!("Unhandled instruction at 0x0 for {:#X}", instruction);
                    }
                }
             
            }

            0x1 => {
                self.program_counter = nnn;
            }

            0x2 => {
                // Call subroutine at nnn.
                self.stack_pointer += 1;
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.program_counter = nnn;
            }

            0x3 => {
                //Skip next instruction if Vx = kk.
                if self.vx[x as usize] == kk {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            } 

            0x6 => {
                 self.vx[x as usize] = kk;
                 self.program_counter += 2;
            }

            0x7 => {
                self.vx[x as usize] += kk;
                self.program_counter += 2;
            }

            0xA => {
                self.i = nnn;
                self.program_counter += 2;
            }

            0xD => {
                //TODO Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                // The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
                self.program_counter += 2;
            }

            0xF => {
                match kk {

                    0x0A => {
                        // TODO Wait for a key press, store the value of the key in Vx.
                        // All execution stops until a key is pressed, then the value of that key is stored in Vx.
                        self.program_counter += 2;
                    }

                    0x1E => {
                        self.i += self.vx[x as usize] as u16;
                        self.program_counter += 2;
                    }

                    _ => {
                        panic!("Unhandled instruction: {:#X}", instruction);
                    }
                }
                
            }

            _ => {
                panic!("Unhandled instruction: {:#X}", instruction);
            }

        }

    }
}