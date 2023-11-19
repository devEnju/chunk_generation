use crate::{
    chunk::{provider::Provider, Chunk},
    octaves::{Notch, Octaves},
    random::Random, seed::Seed,
};

pub struct Nether {
    seed: Seed,
    primary: Octaves<Notch>,
    secondary: Octaves<Notch>,
    tertiary: Octaves<Notch>,
    minor: Octaves<Notch>,
    major: Octaves<Notch>,
    noise: [f64; 425],
    n1: [f64; 256],
    n2: [f64; 256],
    n3: [f64; 256],
    radian: [f64; 17],
}

impl Provider for Nether {
    fn new(seed: &Seed) -> Self {
        let mut random = Random::new(seed.value as u64);

        let primary = Octaves::<Notch>::new(&mut random, 16);
        let secondary = Octaves::<Notch>::new(&mut random, 16);
        let tertiary = Octaves::<Notch>::new(&mut random, 8);
        let minor = Octaves::<Notch>::new(&mut random, 4);
        let major = Octaves::<Notch>::new(&mut random, 4);

        let mut radian = [0.0; 17];

        for (index, var) in radian.iter_mut().enumerate() {
            *var = f64::cos((index as f64 * std::f64::consts::PI * 6.0) / 17.0) * 2.0;
            let mut temp = index;

            if index > 8 {
                temp = 16 - index;
            }
            if temp < 4 {
                temp = 4 - temp;
                *var -= (temp * temp * temp) as f64 * 10.0;
            }
        }

        Nether {
            seed,
            primary,
            secondary,
            tertiary,
            minor,
            major,
            noise: [0.0; 425],
            n1: [0.0; 256],
            n2: [0.0; 256],
            n3: [0.0; 256],
            radian,
        }
    }

    fn generate_chunk(&mut self, coords: (i32, i32)) -> Chunk {
        let mut chunk = Chunk::new(coords);

        self.calculate_noise((chunk.coords.0 * 4, 0, chunk.coords.1 * 4));
        self.create_blocks(&mut chunk);
        self.replace_blocks(&mut chunk, self.seed.random_with_const(coords));

        chunk
    }
}

impl Nether {
    fn calculate_noise(&mut self, pos: (i32, i32, i32)) {
        let l1 = 5;
        let l2 = 17;

        let f1 = 8.55515;
        let f2 = f1 * 80.0;

        for x in 0..l1 {
            for z in 0..l1 {
                self.primary.reset();
                self.secondary.reset();
                self.tertiary.reset();

                for y in 0..l2 {
                    let index = ((x * l1 + z) * l2 + y) as usize;

                    let pos = (pos.0 + x, pos.1 + y, pos.2 + z);

                    let d1 = self.primary.noise_3d(pos.0, pos.1, pos.2, f2, 3.0) * 0.001953125;
                    let d2 = self.secondary.noise_3d(pos.0, pos.1, pos.2, f2, 3.0) * 0.001953125;
                    let d3 = self.tertiary.noise_3d(pos.0, pos.1, pos.2, f1, 4.0) * 0.05 + 0.5;

                    let mut total = d1 + (d2 - d1) * d3;

                    if d3 < 0.0 {
                        total = d1;
                    } else if d3 > 1.0 {
                        total = d2;
                    }
                    total -= self.radian[y as usize];

                    if y > 13 {
                        total -= (total + 10.0) * (y - 13) as f64 / 3.0;
                    }
                    self.noise[index] = total;
                }
            }
        }

        let f1 = 0.03125;
        let f2 = f1 * 2.0;

        for x in 0..l1 - 1 {
            self.minor.reset();
            self.major.reset();

            for z in 0..l1 - 1 {
                let index = (x << 4 | z) as usize;

                let pos = (pos.0 * 4 + x, pos.2 * 4 + z);

                self.n1[index] = self.minor.noise_3d(pos.0, pos.1, 0, f1, 0.5);
                self.n2[index] = self.minor.noise_2d(pos.0, pos.1, f1);
                self.n3[index] = self.major.noise_3d(pos.0, pos.1, 0, f2, 0.5);
            }
        }
    }

    fn create_blocks(&self, chunk: &mut Chunk) {
        let l1 = 5;
        let l2 = 17;

        for x in 0..l1 - 1 {
            for z in 0..l1 - 1 {
                for y in 0..l2 - 1 {
                    let index = (x * l1 + z) * l2 + y;

                    let mut d1 = self.noise[index];
                    let mut d2 = self.noise[index + l2];
                    let mut d3 = self.noise[index + l2 * l1];
                    let mut d4 = self.noise[index + l2 * (l1 + 1)];

                    let d5 = (self.noise[index + 1] - d1) * 0.125;
                    let d6 = (self.noise[index + l2 + 1] - d2) * 0.125;
                    let d7 = (self.noise[index + l2 * l1 + 1] - d3) * 0.125;
                    let d8 = (self.noise[index + l2 * (l1 + 1) + 1] - d4) * 0.125;

                    for v in 0..8 {
                        let mut d10 = d1;
                        let mut d11 = d2;

                        for u in 0..4 {
                            let mut d9 = d10;

                            for w in 0..4 {
                                let x = x << 2 | u;
                                let y = y << 3 | v;
                                let z = z << 2 | w;

                                let mut block = 0; // Block.air

                                if y < 32 {
                                    block = 11; // Block.stillLava
                                }
                                if d9 > 0.0 {
                                    block = 87; // Block.netherrack;
                                }
                                chunk.blocks[x][y][z] = block;

                                d9 += (d11 - d10) * 0.25;
                            }

                            d10 += (d3 - d1) * 0.25;
                            d11 += (d4 - d2) * 0.25;
                        }

                        d1 += d5;
                        d2 += d6;
                        d3 += d7;
                        d4 += d8;
                    }
                }
            }
        }
    }

    fn replace_blocks(&self, chunk: &mut Chunk, mut random: Random) {
        for z in 0..16 {
            for x in 0..16 {
                let index = x << 4 | z;

                let sand = self.n1[index] + random.next_f64() * 0.2 > 0.0;
                let gravel = self.n2[index] + random.next_f64() * 0.2 > 0.0;

                let mut count = 0;
                let amount = (self.n3[index] / 3.0 + 3.0 + random.next_f64() * 0.25) as i32;

                let mut top = if amount > 0 { 87 } else { 0 }; // Block.netherrack : Block.air
                let mut low = 87; // Block.netherrack

                for y in (0..=127).rev() {
                    let index = x << 11 | z << 7 | y;

                    if y as u32 >= 127 - random.next_u32(5) {
                        chunk.blocks[x][y][z] = 7; // Block.bedrock
                    } else if y as u32 <= random.next_u32(5) {
                        chunk.blocks[x][y][z] = 7; // Block.bedrock
                    } else {
                        let current = chunk.blocks[x][y][z];

                        if current == 0 { // Block.air
                            count = 0;
                        } else if current == 87 { // Block.netherrack
                            if count == 0 {
                                if amount > 0 && 59 < y && y < 66 {

                                    if sand {
                                        top = 88; // Block.soulsand
                                        low = 88; // Block.soulsand
                                    } else if gravel {
                                        top = 13; // Block.gravel
                                        low = 87; // Block.netherrack
                                    }
                                }
                                if y < 64 && top == 0 { // Block.air
                                top = 11; // Block.stillLava
                                }
                                count = amount + 1;

                                chunk.blocks[x][y][z] = if y > 62 { top } else { low };
                            } else if count > 1 {
                                count -= 1;
                                chunk.blocks[x][y][z] = low;
                            }
                        }
                    }
                }
            }
        }
    }
}
