pub mod manager;
pub mod provider;

use crate::dimension::Dimension;

use self::provider::Provider;

pub struct Chunk<'a, T: Provider> {
    dimension: &'a Dimension<'a, T>,
    coords: (i32, i32),
    pub blocks: [[[u8; 16]; 128]; 16],
}

impl<'a, T: Provider> Chunk<'a, T> {
    pub fn new(dimension: &'a Dimension<T>, coords: (i32, i32)) -> Self {
        Chunk {
            dimension,
            coords,
            blocks: [[[0; 16]; 128]; 16],
        }
    }
}
