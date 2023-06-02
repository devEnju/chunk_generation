use crate::random::Random;

pub mod perlin;
pub mod simplex;

pub trait Noise {
    fn new(rand: &mut Random) -> Self;
}

fn vector_xy(rand: &mut Random) -> (f64, f64) {
    let x = rand.next_f64() * 256.0;
    let y = rand.next_f64() * 256.0;
    rand.skip_f64();

    (x, y)
}

fn vector_xyz(rand: &mut Random) -> (f64, f64, f64) {
    let x = rand.next_f64() * 256.0;
    let y = rand.next_f64() * 256.0;
    let z = rand.next_f64() * 256.0;

    (x, y, z)
}

fn permutations(rand: &mut Random) -> [usize; 512] {
    let mut array: [usize; 256] = core::array::from_fn(|i| i);
    let mut permutations = [0; 512];

    for i in array {
        let temp = rand.next_u32(256 - i as i32) as usize + i;

        permutations[i] = array[temp];
        array[temp] = array[i];

        permutations[i + 256] = permutations[i];
    }
    permutations
}