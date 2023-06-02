use crate::chunk::{provider::Provider, Chunk};

pub struct Nether {}

impl Provider for Nether {
    fn new() -> Self {
        Nether {}
    }

    fn generate_chunk<T: Provider>(&self) -> Chunk<T> {
        todo!()
    }
}
