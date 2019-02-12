const PI_2:f32 = 2.0 * 3.141592;

pub struct WaveStruct<T: WaveGenerator> {
    pub current_clock: f32,
    sample_rate: f32,
    calc_freq: f32,
    wave_gen: T,
}

pub trait WaveGenerator {
    fn next(&self, clock: f32, sample_rate: f32, freq: f32) -> f32;
}

impl<W: WaveGenerator> WaveStruct<W> {
    pub fn new(sample_rate: f32, freq: f32, wave_gen: W) -> WaveStruct<W> {
        return WaveStruct {
            current_clock: 0.0,
            sample_rate: sample_rate,
            calc_freq: freq,
            wave_gen: wave_gen
        }
    }
    pub fn next(&mut self) -> f32 {
        self.current_clock = (self.current_clock + 1.0) % self.sample_rate;
        return self.wave_gen.next(self.current_clock, self.sample_rate, self.calc_freq);
    }
    pub fn change_freq(&mut self, freq: f32) {
        self.calc_freq = freq;
    }
}

pub struct SineWave {}
impl WaveGenerator for SineWave {
    fn next(&self, clock: f32, sample_rate: f32, freq: f32) -> f32 {
        println!("Freq: {}", freq);
        return (clock * freq * PI_2 / sample_rate).sin();
    }
}