
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

//#[derive(Debug)]
pub struct Display {
    pub screen: [u8; WIDTH * HEIGHT],
    pub clear_screen: bool,
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [0; WIDTH * HEIGHT],
            clear_screen: false,
        }
    }



    pub fn get_index_from_coords(x: usize, y: usize) -> usize {
        (y as u16 * 64 + x as u16) as usize    
    }

    pub fn get_coords_from_index(index: usize) -> (usize, usize) {
        let width = 64;
        let x = index as u16 % width as u16;
        let y = index as u16 / width as u16;
        (x as usize, y as usize)
    }


}