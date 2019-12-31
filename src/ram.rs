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
}