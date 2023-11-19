pub mod manager;
pub mod provider;

pub struct Chunk {
    pub coords: (i32, i32),
    pub blocks: [[[u8; 16]; 128]; 16],
}

impl Chunk {
    pub fn new(coords: (i32, i32)) -> Self {
        Chunk {
            coords,
            blocks: [[[0; 16]; 128]; 16],
        }
    }
}
