use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::keyboard::Keyboard;
use crate::display::Display;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;


//#[derive(Debug)]
pub struct Machine {
    cpu: Cpu,
    bus: Bus,
    keyboard: Keyboard,
    display: Display,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            cpu: Cpu::new(),
            bus: Bus::new(),
            keyboard: Keyboard::new(),
            display: Display::new()
        }
    }

    pub fn load_rom(&mut self, data: Vec<u8>) { //TODO return result
        self.bus.load_rom(data);
    }

    pub fn start(&mut self) { //TODO return result
         

            let (mut ctx, mut event_loop) = ContextBuilder::new("game_name", "author_name")
                                            .build()
                                            .unwrap();

   

            match event::run(&mut ctx, &mut event_loop, self) {
                Ok(_) => println!("Exited cleanly."),
                Err(e) => println!("Error occured: {}", e)
            }

            
        
    }
}

impl EventHandler for Machine {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
       if self.cpu.should_execute {
            self.cpu.execute_instruction(&mut self.bus);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        // Draw code here...

        graphics::present(ctx)
    }
}