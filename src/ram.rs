pub const PROGRAM_START: u16 = 0x200;

//#[derive(Debug)]
pub struct Ram {
    memory: [u8; 4096],
}

impl Ram {
    pub fn new() -> Ram {
        let mut ram = Ram {
            memory: [0; 4096],
        };


        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0],
            [0x20, 0x60, 0x20, 0x20, 0x70],
            [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            [0x90, 0x90, 0xF0, 0x10, 0x10],
            [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            [0xF0, 0x10, 0x20, 0x40, 0x40],
            [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            [0xF0, 0x90, 0xF0, 0x90, 0x90],
            [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            [0xF0, 0x80, 0x80, 0x80, 0xF0],
            [0xE0, 0x90, 0x90, 0x90, 0xE0],
            [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            [0xF0, 0x80, 0xF0, 0x80, 0x80],
        ];

        let mut i = 0;
        for sprite in &sprites {
            for ch in sprite {
                ram.memory[i] = *ch;
                i += 1;
            }
        }

        ram
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        let mut rom_index = 0;
        for i in {PROGRAM_START as usize..(rom.len() + (PROGRAM_START as usize))} {
            self.memory[i as usize] = rom[rom_index];
            rom_index += 1;
        }

    }

    pub fn write_byte(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}