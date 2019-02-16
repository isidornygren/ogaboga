const PI:f32 = 3.141592;
const PI_2:f32 = 2.0 * PI;

pub struct WaveStruct {
    pub current_clock: f32,
    sample_rate: f32,
    calc_freq: f32,
    wave_gen: &'static (Fn(f32) -> f32 + Sync),
    step_size: f32,
    step: f32,
}

pub trait WaveGenerator {
    fn next(&self, clock: f32) -> f32;
}

impl WaveStruct {
    pub fn new(sample_rate: f32, freq: f32, wave_gen: &'static (Fn(f32) -> f32 + Sync)) -> WaveStruct {
        return WaveStruct {
            current_clock: 0.0,
            sample_rate: sample_rate,
            calc_freq: freq,
            wave_gen: wave_gen,
            step_size: (PI_2 * freq) / sample_rate,
            step: 0.0,
        }
    }
    pub fn next(&mut self) -> f32 {
        // Will create a period between 0 and and 2*PI
        self.step = (self.step + self.step_size) % PI_2;
        return (self.wave_gen)(self.step);
    }
    pub fn change_freq(&mut self, freq: f32) {
        self.step_size = (PI_2 * freq) / self.sample_rate;
    }
}

// Some waveform generators, you could just use f32::sin to get a normal sinusoid
pub fn square_wave(clock: f32) -> f32 {
    return (clock / PI_2).round() * 2.0 - 1.0;
}
pub fn sawtooth_wave(clock: f32) -> f32 {
    return clock / PI - 1.0;
}
pub fn triangle_wave(clock: f32) -> f32 {
    return (clock / PI - 1.0).abs();
}