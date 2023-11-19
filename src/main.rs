use random::Random;

use crate::octaves::{Notch, Octaves, Perlin, Simplex};

mod chunk;
mod dimension;
mod math;
mod octaves;
mod random;
mod seed;
mod world;

fn main() {
    math::init();
    
    println!("Hello, world!");

    let mut rand = random::Random::new(64);

    println!("{}", rand.next_u32(128));
    println!("{}", rand.next_u32(128));
    println!("{}", rand.next_u32(128));
    println!("{}", rand.next_u32(128));
    println!("{}", rand.next_u32(128));
    println!("{}", rand.next_i64());
    println!("{}", rand.next_i64());
    println!("{}", rand.next_i64());
    println!("{}", rand.next_f32());
    println!("{}", rand.next_f32());
    println!("{}", rand.next_f32());
    println!("{}", rand.next_u32(1));
    println!("{}", rand.next_u32(3));
    println!("{}", rand.next_u32(5));
    println!("{}", rand.next_u32(7));
    println!("{}", rand.next_u32(11));
    println!("{}", rand.next_f64());
    println!("{}", rand.next_f64());
    println!("{}", rand.next_f64());

    let octaves = Octaves::<Simplex>::new(&mut Random::new(0), 8);
    println!("{}", octaves.noise_2d(26, 5, 1.5, 0.3));

    let octaves = Octaves::<Perlin>::new(&mut Random::new(0), 12);
    println!("{}", octaves.noise_2d(26, 5, 3.0));

    let octaves = Octaves::<Perlin>::new(&mut Random::new(0), 4);
    println!("{}", octaves.noise_3d(26, 5));

    let octaves = Octaves::<Notch>::new(&mut Random::new(0), 6);
    println!("{}", octaves.noise_2d(10, 47, 1.5));

    let mut octaves = Octaves::<Notch>::new(&mut Random::new(0), 3);
    println!("{}", octaves.noise_3d(10, 47, 12, 2.7, 1.5));
}
