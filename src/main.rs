use std::env;
use std::fs::File;
use std::io::Read;


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

    
}
