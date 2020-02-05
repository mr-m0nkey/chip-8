mod machine;
mod cpu;
mod ram;
mod bus;
mod keyboard;
mod display;

use std::env;
use std::fs::File;
use std::io::Read;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler, KeyCode, KeyMods, EventsLoop};
use ggez::graphics;
use machine::Machine;
use ggez::conf::FullscreenType;
use ggez::conf::WindowMode;

const PIXEL_SIZE: f32 = 15.0;



fn main() {
    println!("Hello, this is my chip 8 emulator!");

    let args: Vec<String> = env::args().collect();
    let file_name = match args.len() {
        0 | 1 => {
          panic!("invalid argument length");  
        },
        _ => args.get(1).unwrap(),
    };
    let mut file = File::open(file_name).unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("File not found!");


     let (mut ctx, mut event_loop) = ContextBuilder::new("game_name", "author_name")
                                            .window_mode(
                                                WindowMode{
                                                    width: 64.0 * PIXEL_SIZE,
                                                    height: 32.0 * PIXEL_SIZE,
                                                    maximized: false,
                                                    fullscreen_type: FullscreenType::Windowed,
                                                    borderless: false,
                                                    min_width: 0.0,
                                                    max_width: 0.0,
                                                    min_height: 0.0,
                                                    max_height: 0.0,
                                                    resizable: false,
                                                })
                                            .build()
                                            
                                            .unwrap();

    
   


    let mut machine = Machine::new();
    machine.load_rom(data);
    //TODO refactor into machine.start()
    match event::run(&mut ctx, &mut event_loop, &mut machine) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }

    

    
}
