use crate::seed::Seed;

pub struct World {
    pub seed: Seed,
}

impl World {
    pub fn new(seed: i64) -> Self {
        World {
            seed: Seed::new(seed),
        }
    }
}
