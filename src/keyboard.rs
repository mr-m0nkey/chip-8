use ggez::event::{self, EventHandler, KeyCode, KeyMods, EventsLoop};


//#[derive(Debug)]
pub struct Keyboard {
    
}

impl Keyboard {
    pub fn new () -> Keyboard {
        Keyboard {
            
        }
    }

    pub fn get_u8_from_keycode(keycode: KeyCode) -> Option<u8> {
        match keycode {
            
            KeyCode::Key0 => {
                Some(0x0)
            }

            KeyCode::Key1 => {
                Some(0x1)
            }

            KeyCode::Key2 => {
                Some(0x2)
            }

            KeyCode::Key3 => {
                Some(0x3)
            }

            KeyCode::Key4 => {
                Some(0x4)
            }

            KeyCode::Key5 => {
                Some(0x5)
            }

            KeyCode::Key6 => {
                Some(0x6)
            }

            KeyCode::Key7 => {
                Some(0x7)
            }

            KeyCode::Key8 => {
                Some(0x8)
            }

            KeyCode::Key9 => {
                Some(0x9)
            }

            KeyCode::A => {
                Some(0xA)
            }
            KeyCode::B => {
                Some(0xB)
            }
            KeyCode::C => {
                Some(0xC)
            }
            KeyCode::D => {
                Some(0xD)
            }
            KeyCode::E => {
                Some(0xE)
            }
            KeyCode::F => {
                Some(0xF)
            }

            _ => {
                None
            }
        }
    }
}