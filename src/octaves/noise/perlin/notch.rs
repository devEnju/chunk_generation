use crate::random::Random;

use super::{Noise, Perlin};

pub struct Notch {
    perlin: Perlin,
    pub option: Option<usize>,
    pub lerps: [f64; 4],
}

impl Noise for Notch {
    fn new(rand: &mut Random) -> Self {
        Notch {
            perlin: Perlin::new(rand),
            option: Option::None,
            lerps: [0.0; 4],
        }
    }
}

impl Notch {
    pub fn generate_2d(&self, u: f64, v: f64) -> f64 {
        self.perlin.generate_2d(u, v)
    }

    pub fn generate_3d(&mut self, u: f64, v: f64, w: f64) -> f64 {
        let mut vec = (
            self.perlin.vector.0 + u,
            self.perlin.vector.1 + v,
            self.perlin.vector.2 + w,
        );

        let x = super::floor(vec.0);
        let y = super::floor(vec.1);
        let z = super::floor(vec.2);

        vec = (vec.0 - x as f64, vec.1 - y as f64, vec.2 - z as f64);

        let x = (x & 0xff) as usize;
        let y = (y & 0xff) as usize;
        let z = (z & 0xff) as usize;

        if self.option.map_or(true, |o| o != y) {
            self.option = Some(y);

            let mut index: [usize; 2] =
                core::array::from_fn(|i| self.perlin.permutations[x + i] + y);

            let mut grads = [0.0; 8];

            for i in 0..index.len() {
                for j in 0..index.len() {
                    index[j] += i;

                    let k = self.perlin.permutations[index[j]] + z;

                    grads[i * 2 + j] = super::grad(
                        self.perlin.permutations[k],
                        (vec.0 - j as f64, vec.1 - i as f64, vec.2),
                    );
                    grads[i * 2 + j + 4] = super::grad(
                        self.perlin.permutations[k + 1],
                        (vec.0 - j as f64, vec.1 - i as f64, vec.2 - 1.0),
                    );
                }
            }

            for i in 0..self.lerps.len() {
                self.lerps[i] = super::lerp(vec.0, grads[i * 2], grads[i * 2 + 1]);
            }
        };

        super::lerp(
            vec.2,
            super::lerp(vec.1, self.lerps[0], self.lerps[1]),
            super::lerp(vec.1, self.lerps[2], self.lerps[3]),
        )
    }
}
