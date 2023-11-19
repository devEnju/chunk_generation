use crate::{chunk::{provider::Provider, Chunk}, octaves::{Octaves, Perlin, Notch}, random::Random, seed::Seed};

pub struct Overworld {
    seed: Seed,
    random: Random,
    primary: Octaves<Notch>,
    secondary: Octaves<Notch>,
    tertiary: Octaves<Notch>,
    minor: Octaves<Notch>,
    major: Octaves<Notch>,
    one: Octaves<Perlin>,
    two: Octaves<Perlin>,
    three: Octaves<Perlin>,
}

impl Provider for Overworld {
    fn new(seed: &Seed) -> Self {
        let mut random = Random::new(seed.value as u64);

        let primary = Octaves::<Notch>::new(&mut random, 16);
        let secondary = Octaves::<Notch>::new(&mut random, 16);
        let tertiary = Octaves::<Notch>::new(&mut random, 8);
        let minor = Octaves::<Notch>::new(&mut random, 4);
        let major = Octaves::<Notch>::new(&mut random, 4);
        let one = Octaves::<Perlin>::new(&mut random, 10);
        let two = Octaves::<Perlin>::new(&mut random, 16);
        let three = Octaves::<Perlin>::new(&mut random, 8);

        Overworld {
            seed,
            random,
            primary,
            secondary,
            tertiary,
            minor,
            major,
            one,
            two,
            three,
        }
    }

    fn generate_chunk(&mut self, coords: (i32, i32)) -> Chunk {
        Chunk::new(coords)
    }
}
