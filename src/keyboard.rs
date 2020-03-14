use ggez::event::{self, EventHandler, EventsLoop, KeyCode, KeyMods};

//#[derive(Debug)]
pub struct Keyboard {}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {}
    }

    pub fn get_u8_from_keycode(keycode: KeyCode) -> Option<u8> {
        match keycode {
            KeyCode::Key0 => Some(0x0),

            KeyCode::Key1 => Some(0x1),

            KeyCode::Key2 => Some(0x2),

            KeyCode::Key3 => Some(0x3),

            KeyCode::Key4 => Some(0x4),

            KeyCode::Key5 => Some(0x5),

            KeyCode::Key6 => Some(0x6),

            KeyCode::Key7 => Some(0x7),

            KeyCode::Key8 => Some(0x8),

            KeyCode::Key9 => Some(0x9),

            KeyCode::A => Some(0xA),
            KeyCode::B => Some(0xB),
            KeyCode::C => Some(0xC),
            KeyCode::D => Some(0xD),
            KeyCode::E => Some(0xE),
            KeyCode::F => Some(0xF),

            _ => None,
        }
    }

    pub fn get_keycode_from_u8(int_value: u8) -> Option<KeyCode> {
        match int_value {
            0x0 => Some(KeyCode::Key0),

            0x1 => Some(KeyCode::Key1),

            0x2 => Some(KeyCode::Key2),

            0x3 => Some(KeyCode::Key3),

            0x4 => Some(KeyCode::Key4),

            0x5 => Some(KeyCode::Key5),

            0x6 => Some(KeyCode::Key6),

            0x7 => Some(KeyCode::Key7),

            0x8 => Some(KeyCode::Key8),

            0x9 => Some(KeyCode::Key9),

            0xA => Some(KeyCode::A),

            0xB => Some(KeyCode::B),

            0xC => Some(KeyCode::C),

            0xD => Some(KeyCode::D),

            0xE => Some(KeyCode::E),

            0xF => Some(KeyCode::F),

            _ => None,
        }
    }
}
