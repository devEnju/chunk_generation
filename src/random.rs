const ADD: u64 = 0xb;
const MUL: u64 = 0x5deece66d;
const SHL: u64 = (1 << 48) - 1;
const F32: f32 = (1u32 << 24) as f32;
const F64: f64 = (1u64 << 53) as f64;

pub struct Random {
    seed: u64,
}

impl Random {
    pub fn new(seed: u64) -> Self {
        Random {
            seed: (seed ^ MUL) & SHL,
        }
    }

    fn next(&mut self, bits: u8) -> i32 {
        self.seed = self.seed.wrapping_mul(MUL).wrapping_add(ADD) & SHL;
        (self.seed >> (48 - bits)) as i32
    }

    pub fn next_u32(&mut self, bound: i32) -> u32 {
        let n = u64::try_from(bound).expect("positive number");

        if n.is_power_of_two() {
            return (n.wrapping_mul(self.next(31) as u64) >> 31) as u32;
        }

        let mut bits;
        let mut val;

        loop {
            bits = self.next(31);
            val = bits % bound;

            if bits - val + (bound - 1) >= 0 {
                return val as u32;
            }
        }
    }

    pub fn next_i64(&mut self) -> i64 {
        ((self.next(32) as i64) << 32) + self.next(32) as i64
    }

    pub fn next_f32(&mut self) -> f32 {
        self.next(24) as f32 / F32
    }

    pub fn next_f64(&mut self) -> f64 {
        (((self.next(26) as i64) << 27) + self.next(27) as i64) as f64 / F64
    }

    pub fn skip_u32(&mut self, bound: i32) {
        let n = u64::try_from(bound).expect("positive number");

        if n.is_power_of_two() {
            self.next(31);
        } else {
            let mut bits;
            let mut val;
            
            loop {
                bits = self.next(31);
                val = bits % bound;
    
                if bits - val + (bound - 1) >= 0 {
                    break;
                }
            }
        }
    }

    pub fn skip_i64(&mut self) {
        self.next(32);
        self.next(32);
    }

    pub fn skip_f32(&mut self) {
        self.next(24);
    }

    pub fn skip_f64(&mut self) {
        self.next(26);
        self.next(27);
    }
}
