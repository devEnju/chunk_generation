use crate::random::Random;

mod noise;

pub use self::noise::{
    perlin::{notch::Notch, Perlin},
    simplex::Simplex,
};

use self::noise::Noise;

pub struct Octaves<T: Noise> {
    octaves: Vec<T>,
}

impl<T: Noise> Octaves<T> {
    pub fn new(random: &mut Random, iterations: usize) -> Self {
        Octaves {
            octaves: {
                let mut v = Vec::with_capacity(iterations);

                for _ in 0..iterations {
                    v.push(T::new(random));
                }
                v
            },
        }
    }
}

impl Octaves<Simplex> {
    pub fn noise_2d(&self, x: i32, z: i32, frequency: f64, lacunarity: f64) -> f64 {
        let mut total = 0.0;
        let mut frequency = frequency;
        let mut amplitude = 1.0;

        for noise in &self.octaves {
            total += amplitude * noise.generate_2d(frequency * x as f64, frequency * z as f64);
            frequency *= lacunarity;
            amplitude *= 2.0;
        }
        total
    }
}

impl Octaves<Perlin> {
    pub fn noise_2d(&self, x: i32, z: i32, frequency: f64) -> f64 {
        let mut total = 0.0;
        let mut frequency = frequency;
        let mut amplitude = 1.0;

        for noise in &self.octaves {
            total += amplitude * noise.generate_2d(frequency * x as f64, frequency * z as f64);
            frequency *= 0.5;
            amplitude *= 2.0;
        }
        total
    }

    pub fn noise_3d(&self, x: i32, z: i32) -> f64 {
        let mut total = 0.0;
        let mut frequency = 0.5;
        let mut amplitude = 1.0;

        for noise in &self.octaves {
            total += amplitude * noise.generate_3d(frequency * x as f64, frequency * z as f64);
            frequency *= 0.5;
            amplitude *= 2.0;
        }
        total
    }
}

impl Octaves<Notch> {
    pub fn reset(&mut self) {
        for octave in &mut self.octaves {
            octave.option = Option::None;
            octave.lerps = [0.0; 4];
        }
    }

    pub fn noise_2d(&self, x: i32, z: i32, frequency: f64) -> f64 {
        let mut total = 0.0;
        let mut frequency = frequency;
        let mut amplitude = 1.0;

        for noise in &self.octaves {
            total += amplitude * noise.generate_2d(frequency * x as f64, frequency * z as f64);
            frequency *= 0.5;
            amplitude *= 2.0;
        }
        total
    }

    pub fn noise_3d(&mut self, x: i32, y: i32, z: i32, frequency: f64, height: f64) -> f64 {
        let mut total = 0.0;
        let mut frequency = frequency;
        let mut amplitude = 1.0;

        for noise in &mut self.octaves {
            total += amplitude
                * noise.generate_3d(
                    frequency * x as f64,
                    frequency * y as f64 * height,
                    frequency * z as f64,
                );
            frequency *= 0.5;
            amplitude *= 2.0;
        }
        total
    }
}
