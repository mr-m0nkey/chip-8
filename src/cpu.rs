use crate::bus::Bus;
use crate::ram::PROGRAM_START;
use ggez::input::keyboard;
use ggez::{Context, ContextBuilder, GameResult};


const INSTRUCTION_LENGTH: u16 = 2;

//#[derive(Debug)]
pub struct Cpu {
    v: [u8; 16],
    i: u16,
    delay_timer_register: u8,
    sound_timer_register: u8,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 16],
    pub should_execute: bool,
    pub waiting_for_keypress: bool,
    //TODO add bus reference as a property, read about lifetimes
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            v: [0; 16],
            i: 0,
            delay_timer_register: 0,
            sound_timer_register: 0,
            program_counter: PROGRAM_START,
            stack_pointer: 0,
            stack: [0;16],
            should_execute: true,
            waiting_for_keypress: false,
        }
    }

    pub fn execute_instruction(&mut self, bus: &mut Bus, context: &mut Context) { 
        let most_significant_byte = bus.read_byte(self.program_counter) as u16;
        let least_significant_byte = bus.read_byte(self.program_counter + 1) as u16;
        let instruction: u16 = (most_significant_byte << 8) | least_significant_byte;

        let nnn = instruction & 0x0FFF;
        let n =  (instruction & 0xF) as u8;
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;
        let kk = (instruction & 0xFF) as u8;
        println!("Current executing instrcution: {:#X}", instruction);
        println!("Program counter: {}", self.program_counter);

        match (instruction & 0xF000) >> 12 {
            0x0 => {
                match kk {
                    0xE0 => {
                        //TODO clear the display
                        self.program_counter += INSTRUCTION_LENGTH;
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
                if self.v[x as usize] == kk {
                    self.program_counter += 4;
                } else {
                    self.program_counter += INSTRUCTION_LENGTH;
                }
            } 

            0x6 => {
                 self.v[x as usize] = kk;
                 self.program_counter += INSTRUCTION_LENGTH;
            }

            0x7 => {
                self.v[x as usize] = self.v[x as usize].wrapping_add(kk);// += kk;
                self.program_counter += INSTRUCTION_LENGTH;
            }

            0x8 => {
               
               match n {
                   0x0 => {
                       self.v[x as usize] = self.v[y as usize];
                       self.program_counter += INSTRUCTION_LENGTH;
                   }

                   0x6 => {
                        // Set Vx = Vx SHR 1.
                        // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
                        self.v[0xF as usize] = self.v[x as usize] & 0x1;
                        self.v[x as usize] = self.v[x as usize] >> 1;
                        self.program_counter += INSTRUCTION_LENGTH;
                   }

                   _ => {
                       panic!("Unhandled instruction: {:#X}", instruction);
                   }
               }
            }

            0xA => {
                self.i = nnn;
                self.program_counter += INSTRUCTION_LENGTH;
            }

            0xD => {
                //TODO Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                // The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
                self.program_counter += INSTRUCTION_LENGTH;
            }

            0xF => {
                match kk {

                    0x0A => {
                        if keyboard::pressed_keys(context).len() > 0 {
                            // TODO Wait for a key press, store the value of the key in Vx.
                            // All execution stops until a key is pressed, then the value of that key is stored in Vx.
                            self.program_counter += INSTRUCTION_LENGTH;
                        } 

                       
                       
                     
                    }

                    0x1E => {
                        self.i += self.v[x as usize] as u16;
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    0x29 => {
                        // TODO Set I = location of sprite for digit Vx.
                        // The value of I is set to the location for the hexadecimal sprite 
                        // corresponding to the value of Vx. See section 2.4, 
                        // Display, for more information on the Chip-8 hexadecimal font.
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    0x33 => {
                        let vx = self.v[x as usize];
                        bus.write_byte(self.i as u16, vx / 100);
                        bus.write_byte((self.i + 1) as u16, (vx % 100) / 10);
                        bus.write_byte((self.i + 2) as u16, vx % 10);
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    0x65 => {
                        // The interpreter reads values from memory starting at location I into registers V0 through Vx.
                        let mut counter = 0;
                        for _ in 0..self.v.len() {
                            self.v[counter as usize] = bus.read_byte(self.i + counter);
                            counter += 1;
                        }
                        self.program_counter += INSTRUCTION_LENGTH;
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