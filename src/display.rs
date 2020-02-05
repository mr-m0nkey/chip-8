
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

//#[derive(Debug)]
pub struct Display {
    pub screen: [u8; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [0; WIDTH * HEIGHT],
        }
    }

    pub fn get_index_from_coords(x: usize, y: usize) -> usize {
        y * WIDTH + x
    }

    pub fn get_coords_from_index(index: usize) -> (usize, usize) {
        unimplemented!();
    }


}