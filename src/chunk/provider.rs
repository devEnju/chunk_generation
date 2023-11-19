use crate::seed::Seed;

use super::Chunk;

pub trait Provider {
    fn new(seed: &Seed) -> Self;

    fn generate_chunk(&mut self, coords: (i32, i32)) -> Chunk;

    fn load_chunk() -> Chunk {
        Chunk::new((0, 0))
    }

    fn save_chunk<T: Provider>(&self, chunk: Chunk) {
        todo!()
    }
}
