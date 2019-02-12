use super::wave_generator::{WaveStruct, WaveGenerator};
use super::envelope::Envelope;

pub struct PulseModulator<W: WaveGenerator> {
    clock: f32,
    sample_rate: f32,
    envelope: Envelope,
    wave_form: WaveStruct<W>,
}

impl <W: WaveGenerator> PulseModulator <W> {
    pub fn new(sample_rate: f32, envelope: Envelope, wave_form: WaveStruct<W>) -> PulseModulator<W> {
        return PulseModulator {
            clock: 0.0,
            sample_rate: sample_rate,
            envelope: envelope,
            wave_form: wave_form
        }
    }
    fn start(&mut self){
        self.clock = 0.0;
    }
    fn stop(&mut self){
        // implement later
    }
    fn pulse(&mut self){
        self.start();
        self.stop();
    }
    pub fn next(&mut self) -> f32 {
        self.clock = self.clock + 1.0;
        // time in seconds
        let seconds = self.clock / self.sample_rate;
        let amplitude = self.envelope.get_amplitude(seconds);
        let freq = amplitude * 100.0;
        self.wave_form.change_freq(freq + 440.0);
        // println!("{}", amplitude);
        return self.wave_form.next();
    }
}