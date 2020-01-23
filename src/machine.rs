use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::keyboard::Keyboard;
use crate::display::Display;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler, KeyCode, KeyMods, EventsLoop};
use ggez::input::keyboard;
use ggez::graphics;


//#[derive(Debug)]
pub struct Machine {
    cpu: Cpu,
    pub bus: Bus,
    keyboard: Keyboard,
    display: Display,
}



impl EventHandler for Machine {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
       if self.cpu.should_execute {
            self.cpu.execute_instruction(&mut self.bus, _ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        // Draw code here...

        graphics::present(ctx)
    }



    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, _: bool) {

        match key {
            // Quit if Shift+Ctrl+Q is pressed.
            KeyCode::Q => {
                if mods.contains(KeyMods::SHIFT & KeyMods::CTRL) {
                    println!("Terminating!");
                    event::quit(ctx);
                } else if mods.contains(KeyMods::SHIFT) || mods.contains(KeyMods::CTRL) {
                    println!("You need to hold both Shift and Control to quit.");
                } else {
                    println!("Now you're not even trying!");
                }
            }
            _ => (),
        }
    }
}





impl Machine {
    pub fn new() -> Machine {
        
        
        Machine {
            cpu: Cpu::new(),
            bus: Bus::new(),
            keyboard: Keyboard::new(),
            display: Display::new(),
        }
    }

    pub fn load_rom(&mut self, data: Vec<u8>) { //TODO return result
        self.bus.load_rom(data);
    }

    pub fn start(&mut self) { //TODO return result
         

            

            
        
    }
}