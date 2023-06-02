use crate::chunk::{provider::Provider, Chunk};

pub struct Overworld {}

impl Provider for Overworld {
    fn new() -> Self {
        Overworld {}
    }

    fn generate_chunk<T: Provider>(&self) -> Chunk<T> {
        todo!()
    }
}
