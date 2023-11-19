use crate::random::Random;

pub struct Seed {
    pub value: i64,
    tuple: (i64, i64),
}

impl Seed {
    const TUPLE: (i64, i64) = (0x4f9939f508, 0x1ef1565bd5);

    pub fn new(value: i64) -> Self {
        Seed {
            value,
            tuple: {
                let mut rand = Random::new(value as u64);
                (rand.next_i64() | 1, rand.next_i64() | 1)
            },
        }
    }

    pub fn random_with_value(&self, coords: (i32, i32)) -> Random {
        let (x, z) = coords;
        let (u, v) = self.tuple;

        Random::new((x as i64 * u + z as i64 * v ^ self.value) as u64)
    }

    pub fn random_with_const(&self, coords: (i32, i32)) -> Random {
        let (x, z) = coords;
        let (u, v) = Self::TUPLE;

        Random::new((x as i64 * u + z as i64 * v) as u64)
    }
}
