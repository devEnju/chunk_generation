pub mod nether;
pub mod overworld;

use crate::{
    chunk::{manager::Manager, provider::Provider},
    world::World,
};

pub struct Dimension<'a, T: Provider> {
    world: &'a World,
    manager: Manager<T>,
}

impl<'a, T: Provider> Dimension<'a, T> {
    pub fn new(world: &'a World) -> Self {
        Dimension {
            world,
            manager: Manager::new(T::new(&world.seed)),
        }
    }
}
