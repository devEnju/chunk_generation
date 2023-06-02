use crate::dimension::Dimension;

use super::Chunk;

pub trait Provider {
    fn new() -> Self;

    fn generate_chunk<T: Provider>(&self) -> Chunk<T>;

    fn load_chunk<'a, T: Provider>(&'a self, dimension: &'a Dimension<T>) -> Chunk<T> {
        Chunk::new(dimension, (0, 0))
    }

    fn save_chunk<T: Provider>(&self, chunk: Chunk<T>) {
        todo!()
    }
}
