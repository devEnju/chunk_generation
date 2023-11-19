// #![feature(lazy_cell)]

static mut TRIG: [f32; 65536] = [0.0; 65536];
static mut SQRT: [f64; 64] = [0.0; 64];

pub fn init() {
    unsafe {
        for (index, var) in TRIG.iter_mut().enumerate() {
            *var = f64::sin((index as f64 * std::f64::consts::PI * 2.0) / 65536.0) as f32;
        }
        for (index, var) in SQRT.iter_mut().enumerate() {
            *var = f64::sqrt(index as f64 + 1.0);
        }
    }
}

pub fn sin(f: f32) -> f32 {
    unsafe { TRIG[(f * 10430.38) as usize & 0xffff] }
}

pub fn cos(f: f32) -> f32 {
    unsafe { TRIG[(f * 10430.38 + 16384.0) as usize & 0xffff] }
}

pub fn sqrt(i: usize) -> f64 {
    unsafe { SQRT[i - 1] }
}