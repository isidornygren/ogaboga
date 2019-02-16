const PI:f32 = 3.141592;
const PI_2:f32 = 2.0 * PI;

pub struct WaveStruct {
    current_clock: f32,
    sample_rate: u32,
    wave_gen: &'static (Fn(f32) -> f32 + Sync),
    step_size: f32,
    current_step: f32,
}

impl WaveStruct {
    pub fn new(sample_rate: u32, freq: f32, wave_gen: &'static (Fn(f32) -> f32 + Sync)) -> WaveStruct {
        let mut wave_struct = WaveStruct {
            current_clock: 0.0,
            sample_rate: sample_rate,
            wave_gen: wave_gen,
            step_size: 0.0,
            current_step: 0.0,
        };
        wave_struct.set_freq(freq);
        return wave_struct;
    }
    pub fn next(&mut self) -> f32 {
        // Will create a period between 0 and and 2*PI
        self.current_step = (self.current_step + self.step_size) % PI_2;
        return (self.wave_gen)(self.current_step);
    }
    pub fn set_wave_gen(&mut self, wave_gen: &'static (Fn(f32) -> f32 + Sync)) {
        self.wave_gen = wave_gen;
    }
    pub fn set_freq(&mut self, freq: f32) {
        self.step_size = (PI_2 * freq) / self.sample_rate as f32;
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
    // Triangle wave is just a glorified sawtooth wave
    return (clock / PI - 1.0).abs();
}