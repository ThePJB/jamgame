#[derive(Debug)]
pub struct Sound {
    pub id: usize,
    pub wait: f32,
    pub birthtime: f32,
    pub elapsed: f32,
    pub remaining: f32,
    pub magnitude: f32,
    pub mag_exp: f32,
    pub frequency: f32,
    pub freq_exp: f32,
    pub phase: f32,
    pub samp: u64,
}

pub fn fail_sound(t: f32) -> Sound {
    Sound { id: 1, birthtime: t, elapsed: 0.0, remaining: 0.2, magnitude: 0.2, mag_exp: 0.9995, frequency: 110.0, freq_exp: 1.0, wait: 0.0, phase: 0.0, samp: 0 }
}