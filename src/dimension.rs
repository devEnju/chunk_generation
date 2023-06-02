pub mod nether;
pub mod overworld;

use crate::{
    chunk::{manager::Manager, provider::Provider},
    random::Random,
    world::World,
};

pub struct Dimension<'a, T: Provider> {
    world: &'a World,
    pub random: Random,
    manager: Manager<T>,
}

impl<'a, T: Provider> Dimension<'a, T> {
    pub fn new(world: &'a World) -> Self {
        Dimension {
            world,
            random: Random::new(world.seed.value as u64),
            manager: Manager::new(T::new()),
        }
    }
}
