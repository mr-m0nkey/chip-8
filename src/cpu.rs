use crate::bus::Bus;
use crate::keyboard::Keyboard;
use crate::ram::PROGRAM_START;
use ggez;
use ggez::input::keyboard;
use ggez::Context;
use rand::Rng;

const INSTRUCTION_LENGTH: u16 = 2;

//#[derive(Debug)]
pub struct Cpu {
    v: [u8; 16],
    i: u16,
    pub delay_timer_register: u8,
    sound_timer_register: u8,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 16],
    pub should_execute: bool,
    pub waiting_for_keypress: bool,
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
            stack: [0; 16],
            should_execute: true,
            waiting_for_keypress: false,
        }
    }

    //FIXME there's a bug somewhere here
    pub fn execute_instruction(&mut self, bus: &mut Bus, context: &mut Context) {
        let most_significant_byte = bus.read_byte(self.program_counter) as u16;
        let least_significant_byte = bus.read_byte(self.program_counter + 1) as u16;
        let instruction: u16 = (most_significant_byte << 8) | least_significant_byte;

        let nnn = instruction & 0x0FFF;
        let n = (instruction & 0xF) as u8;
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;
        let kk = (instruction & 0xFF) as u8;
        println!("Current executing instrcution: {:#X}", instruction);
        println!("Program counter: {}", self.program_counter);

        match (instruction & 0xF000) >> 12 {
            0x0 => match kk {
                0xE0 => {
                    bus.display.clear_screen();
                    self.program_counter += INSTRUCTION_LENGTH;
                }
                0xEE => {
                    self.program_counter = self.stack[self.stack_pointer as usize];
                    self.stack_pointer -= 1;
                }
                _ => {
                    panic!("Unhandled instruction at 0x0 for {:#X}", instruction);
                }
            },

            0x1 => {
                self.program_counter = nnn;
            }

            0x2 => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer as usize] = self.program_counter + INSTRUCTION_LENGTH;
                self.program_counter = nnn;
                println!("{}", self.program_counter);
            }

            0x3 => {
                if self.v[x as usize] == kk {
                    self.program_counter += INSTRUCTION_LENGTH * 2;
                } else {
                    self.program_counter += INSTRUCTION_LENGTH;
                }
            }

            0x4 => {
                if self.v[x as usize] != kk {
                    self.program_counter += INSTRUCTION_LENGTH * 2;
                } else {
                    self.program_counter += INSTRUCTION_LENGTH;
                }
            }

            0x5 => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.program_counter += INSTRUCTION_LENGTH * 2;
                } else {
                    self.program_counter += INSTRUCTION_LENGTH;
                }
            }

            0x6 => {
                self.v[x as usize] = kk;
                self.program_counter += INSTRUCTION_LENGTH;
            }

            0x7 => {
                self.v[x as usize] = self.v[x as usize].wrapping_add(kk); // += kk;
                self.program_counter += INSTRUCTION_LENGTH;
            }

            0x8 => {
                match n {
                    0x0 => {
                        self.v[x as usize] = self.v[y as usize];
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    0x1 => {
                        self.v[x as usize] |= self.v[y as usize];
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    0x2 => {
                        self.v[x as usize] &= self.v[y as usize];
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    0x3 => {
                        self.v[x as usize] ^= self.v[y as usize];
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    0x4 => {
                        let sum: u16 = self.v[x as usize] as u16 + self.v[y as usize] as u16;
                        self.v[x as usize] = sum as u8;
                        if sum > 0xFF {
                            self.v[0xF as usize] = 1;
                        }
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    0x5 => {
                        if self.v[x as usize] > self.v[y as usize] {
                            self.v[0xF as usize] = 1;
                        } else {
                            self.v[0xF as usize] = 0;
                        }
                        self.v[x as usize].checked_sub(self.v[y as usize]);
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    0x6 => {
                        // Set Vx = Vx SHR 1.
                        // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
                        // self.v[0xF as usize] = self.v[x as usize] & 0x1;
                        // self.v[x as usize] /= 2;
                        self.v[x as usize] >>= 0x1;
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    0x7 => {
                        if self.v[y as usize] > self.v[x as usize] {
                            self.v[0xF as usize] = 1;
                        } else {
                            self.v[0xF as usize] = 0;
                        }
                        self.v[x as usize] = self.v[y as usize] - self.v[x as usize];
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    0xE => {
                        self.v[x as usize] <<= 0x1;
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    _ => {
                        panic!("Unhandled instruction: {:#X}", instruction);
                    }
                }
            }

            0x9 => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.program_counter += INSTRUCTION_LENGTH;
                    self.program_counter += INSTRUCTION_LENGTH;
                } else {
                    self.program_counter += INSTRUCTION_LENGTH;
                }
            }

            0xA => {
                self.i = nnn;
                self.program_counter += INSTRUCTION_LENGTH;
            }

            0xB => {
                self.program_counter = nnn + self.v[0 as usize] as u16;
            }

            0xC => {
                let random_number = rand::thread_rng().gen_range(0, 255);
                self.v[x as usize] = random_number & kk;
                self.program_counter += INSTRUCTION_LENGTH;
            }

            0xD => {
                let mut counter = 0;
                let mut erased = false;
                for i in self.i..(self.i + n as u16) {
                    let byte = bus.read_byte(i);
                    if bus.draw_byte(byte, self.v[x as usize], self.v[y as usize] + counter) {
                        erased = true;
                    }
                    counter += 1;
                }

                if erased {
                    self.v[0xF as usize] = 1;
                } else {
                    self.v[0xF as usize] = 0;
                }

                self.program_counter += INSTRUCTION_LENGTH;
            }

            0xE => match kk {
                0xA1 => {
                    let key_code_result = Keyboard::get_keycode_from_u8(self.v[x as usize]);
                    match key_code_result {
                        Some(key_code) => {
                            if !keyboard::is_key_pressed(context, key_code) {
                                self.program_counter += INSTRUCTION_LENGTH * 2;
                            } else {
                                self.program_counter += INSTRUCTION_LENGTH;
                            }
                        }

                        _ => {}
                    }
                }

                0x9E => {
                    let key_code_result = Keyboard::get_keycode_from_u8(self.v[x as usize]);
                    match key_code_result {
                        Some(key_code) => {
                            if keyboard::is_key_pressed(context, key_code) {
                                self.program_counter += INSTRUCTION_LENGTH;
                                self.program_counter += INSTRUCTION_LENGTH;
                            } else {
                                self.program_counter += INSTRUCTION_LENGTH;
                            }
                        }

                        _ => {}
                    }
                }

                _ => {
                    panic!("Unhandled instruction: {:#X}", instruction);
                }
            },

            0xF => {
                match kk {
                    0x0A => {
                        let pressed_keys = keyboard::pressed_keys(context);
                        if pressed_keys.len() > 0 {
                            for pressed_key in pressed_keys.iter() {
                                match Keyboard::get_u8_from_keycode(*pressed_key) {
                                    Some(key_value) => {
                                        self.v[x as usize] = key_value;
                                        self.program_counter += INSTRUCTION_LENGTH;
                                        break;
                                    }
                                    None => {}
                                }
                            }
                        }
                    }

                    0x07 => {
                        self.v[x as usize] = self.delay_timer_register;
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    0x15 => {
                        self.delay_timer_register = self.v[x as usize];
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    0x18 => {
                        self.sound_timer_register = self.v[x as usize];
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    0x1E => {
                        self.i += self.v[x as usize] as u16;
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    0x29 => {
                        // Set I = location of sprite for digit Vx.
                        // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
                        self.i = self.v[x as usize] as u16 * 5;
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    0x33 => {
                        let vx = self.v[x as usize];
                        bus.write_byte(self.i as u16, vx / 100);
                        bus.write_byte((self.i + 1) as u16, (vx % 100) / 10);
                        bus.write_byte((self.i + 2) as u16, vx % 10);
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    0x55 => {
                        for index in 0..x {
                            bus.write_byte(self.i, self.v[index as usize]);
                        }
                        self.program_counter += INSTRUCTION_LENGTH;
                    }

                    0x65 => {
                        for counter in 0..self.v.len() {
                            self.v[counter as usize] = bus.read_byte(self.i + counter as u16);
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
