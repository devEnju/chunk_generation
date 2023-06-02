use crate::random::Random;

use super::Noise;

pub mod notch;

pub struct Perlin {
    vector: (f64, f64, f64),
    permutations: [usize; 512],
}

impl Noise for Perlin {
    fn new(rand: &mut Random) -> Self {
        Perlin {
            vector: super::vector_xyz(rand),
            permutations: super::permutations(rand),
        }
    }
}

impl Perlin {
    pub fn generate_2d(&self, u: f64, v: f64) -> f64 {
        let mut vec = (self.vector.0 + u, self.vector.2 + v);

        let x = floor(vec.0);
        let z = floor(vec.1);

        vec = (vec.0 - x as f64, vec.1 - z as f64);

        let x = (x & 0xff) as usize;
        let z = (z & 0xff) as usize;

        let mut index: [usize; 2] = core::array::from_fn(|i| self.permutations[self.permutations[x + i]] + z);

        let mut grads = [0.0; 4];

        for i in 0..index.len() {
            for j in 0..index.len() {
                index[j] += i;

                grads[i * 2 + j] = grad(
                    self.permutations[index[j]],
                    (vec.0 - j as f64, 0.0 ,vec.1 - i as f64),
                );
            }
        }

        let lerps: [f64; 2] = core::array::from_fn(|i| lerp(vec.0, grads[i * 2], grads[i * 2 + 1]));

        lerp(vec.1, lerps[0], lerps[1])
    }

    pub fn generate_3d(&self, u: f64, v: f64) -> f64 {
        let mut vec = (self.vector.0 + u, self.vector.1 + v, self.vector.2);

        let x = floor(vec.0);
        let y = floor(vec.1);
        let z = floor(vec.2);

        vec = (vec.0 - x as f64, vec.1 - y as f64, vec.2 - z as f64);

        let x = (x & 0xff) as usize;
        let y = (y & 0xff) as usize;
        let z = (z & 0xff) as usize;

        let mut index: [usize; 2] = core::array::from_fn(|i| self.permutations[x + i] + y);

        let mut grads = [0.0; 8];

        for i in 0..index.len() {
            for j in 0..index.len() {
                index[j] += i;

                let k = self.permutations[index[j]] + z;

                grads[i * 2 + j] = grad(
                    self.permutations[k],
                    (vec.0 - j as f64, vec.1 - i as f64, vec.2),
                );
                grads[i * 2 + j + 4] = grad(
                    self.permutations[k + 1],
                    (vec.0 - j as f64, vec.1 - i as f64, vec.2 - 1.0),
                );
            }
        }

        let lerps: [f64; 4] = core::array::from_fn(|i| lerp(vec.0, grads[i * 2], grads[i * 2 + 1]));

        lerp(
            vec.2,
            lerp(vec.1, lerps[0], lerps[1]),
            lerp(vec.1, lerps[2], lerps[3]),
        )
    }
}

fn floor(a: f64) -> i32 {
    if a < (a as i32) as f64 {
        return (a - 1.0) as i32;
    }
    a as i32
}

fn grad(permutation: usize, vec: (f64, f64, f64)) -> f64 {
    let hash = permutation & 0xf;
    let u = if hash < 8 { vec.0 } else { vec.1 };
    let v = if hash < 4 {
        vec.1
    } else if hash != 12 && hash != 14 {
        vec.2
    } else {
        vec.0
    };
    (if hash & 0b1 == 0 { u } else { -u }) + (if hash & 0b10 == 0 { v } else { -v })
}

fn lerp(w: f64, a: f64, b: f64) -> f64 {
    (w * (w * 6.0 - 15.0) + 10.0) * (b - a) * w * w * w + a
}
