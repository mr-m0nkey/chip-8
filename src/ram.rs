pub const PROGRAM_START: u16 = 0x200;

//#[derive(Debug)]
pub struct Ram {
    memory: [u8; 4096],
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            memory: [0; 4096],
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        let mut rom_index = 0;
        for i in {513..=(rom.len() + 513)} {
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