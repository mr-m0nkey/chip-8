use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::display::Display;
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::{Context, GameResult};

use ggez::graphics;
use ggez::graphics::DrawParam;

const PIXEL_SIZE: f32 = 15.0;

//#[derive(Debug)]
pub struct Machine {
    cpu: Cpu,
    pub bus: Bus,
}

impl EventHandler for Machine {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.cpu.delay_timer_register > 0 {
            self.cpu.delay_timer_register -= 1;
        } else {
            if self.cpu.should_execute {
                self.cpu.execute_instruction(&mut self.bus, _ctx);
            }
        }

        Ok(())
    }

    //FIXME fix bug with drawing sprites at screen boundaries
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let mut index = 0;
        let mut was_anything_drawn = false;
        for pixel_value in self.bus.display.screen.iter() {
            match pixel_value {
                0x1 => {
                    let (x, y) = Display::get_coords_from_index(index);
                    let rect = graphics::Rect::new(
                        x as f32 * PIXEL_SIZE,
                        y as f32 * PIXEL_SIZE,
                        15.0,
                        15.0,
                    );
                    let r2 = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::stroke(1.0),
                        rect,
                        graphics::Color::new(10.0, 10.0, 10.0, 10.0),
                    )?;
                    graphics::draw(ctx, &r2, DrawParam::default())?;
                    was_anything_drawn = true;
                }
                _ => {}
            }

            index += 1;
        }
        if was_anything_drawn == false {
            graphics::clear(ctx, graphics::BLACK);
        } //TODO use display.is_screen_blank

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
        }
    }

    pub fn load_rom(&mut self, data: Vec<u8>) {
        //TODO return result
        self.bus.load_rom(data);
    }
}
