const PI_2:f32 = 2.0 * 3.141592;

pub struct WaveStruct<T: WaveGenerator> {
    pub current_clock: f32,
    sample_rate: f32,
    calc_freq: f32,
    wave_gen: T,
    step_size: f32,
    step: f32,
}

pub trait WaveGenerator {
    fn next(&self, clock: f32) -> f32;
}

impl<W: WaveGenerator> WaveStruct<W> {
    pub fn new(sample_rate: f32, freq: f32, wave_gen: W) -> WaveStruct<W> {
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
        self.step = (self.step + self.step_size) % PI_2;
        return self.wave_gen.next(self.step);
    }
    pub fn change_freq(&mut self, freq: f32) {
        self.step_size = (PI_2 * freq) / self.sample_rate;
    }
}

pub struct SineWave {}
impl WaveGenerator for SineWave {
    // period is always 2 PI
    fn next(&self, clock: f32) -> f32 {
        return (clock).sin();
    }
}