use crate::random::Random;

use super::Noise;

const F2D: f64 = 0.3660254037844386;
const G2D: f64 = 0.21132486540518713;
const GRADIENTS: [[f64; 3]; 12] = [
    [1.0, 1.0, 0.0],
    [-1.0, 1.0, 0.0],
    [1.0, -1.0, 0.0],
    [-1.0, -1.0, 0.0],
    [1.0, 0.0, 1.0],
    [-1.0, 0.0, 1.0],
    [1.0, 0.0, -1.0],
    [-1.0, 0.0, -1.0],
    [0.0, 1.0, 1.0],
    [0.0, -1.0, 1.0],
    [0.0, 1.0, -1.0],
    [0.0, -1.0, -1.0],
];

pub struct Simplex {
    vector: (f64, f64),
    permutations: [usize; 512],
}

impl Noise for Simplex {
    fn new(rand: &mut Random) -> Self {
        Simplex {
            vector: super::vector_xy(rand),
            permutations: super::permutations(rand),
        }
    }
}

impl Simplex {
    pub fn generate_2d(&self, u: f64, v: f64) -> f64 {
        let vec = (self.vector.0 + u, self.vector.1 + v);

        let mut temp = (vec.0 + vec.1) * F2D;

        let x = floor(vec.0 + temp);
        let y = floor(vec.1 + temp);

        temp = (x + y) as f64 * G2D;

        let vec1 = (vec.0 + (temp - x as f64), vec.1 + (temp - y as f64));

        let x = (x & 0xff) as usize;
        let y = (y & 0xff) as usize;

        let mut i = 0;
        let mut j = 1;

        if vec1.0 > vec1.1 {
            i = 1;
            j = 0;
        }

        let vec2 = (vec1.0 + G2D - i as f64, vec1.1 + G2D - j as f64);
        let vec3 = (vec1.0 + G2D * 2.0 - 1.0, vec1.1 + G2D * 2.0 - 1.0);

        let h1 = self.permutations[x + self.permutations[y]] % 12;
        let h2 = self.permutations[x + i + self.permutations[y + j]] % 12;
        let h3 = self.permutations[x + 1 + self.permutations[y + 1]] % 12;

        let c1 = corner(vec1, h1);
        let c2 = corner(vec2, h2);
        let c3 = corner(vec3, h3);

        38.5 * (c1 + c2 + c3)
    }
}

fn floor(a: f64) -> i32 {
    if a > 0.0 {
        return a as i32;
    }
    (a - 1.0) as i32
}

fn corner(vec: (f64, f64), grad: usize) -> f64 {
    let mut temp = 0.5 - vec.0 * vec.0 - vec.1 * vec.1;

    if temp >= 0.0 {
        temp *= temp;
        return temp * temp * dot(vec, &GRADIENTS[grad]);
    }
    0.0
}

fn dot(vec: (f64, f64), grad: &[f64; 3]) -> f64 {
    vec.0 * grad[0] + vec.1 * grad[1]
}
