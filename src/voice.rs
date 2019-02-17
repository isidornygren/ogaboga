use crate::wave_generator::{WaveGenerator}; 
use crate::pulse_modulator::PulseModulator;

use crate::Envelope;

pub struct Voice {
    wave_gen: WaveGenerator,
    pulse_modulator: PulseModulator,
}

impl Voice {
    pub fn new(sample_rate: u32, envelope: Envelope, waveform: &'static (Fn(f32) -> f32 + Sync)) -> Voice {
        return Voice {
            wave_gen: WaveGenerator::new(sample_rate, 440.0, waveform),
            pulse_modulator: PulseModulator::new(envelope, sample_rate),
        }
    }
    pub fn next(&mut self) -> f32 {
        let amplitude = self.pulse_modulator.next();
        let current_wave = self.wave_gen.next();
        return amplitude * current_wave;
    }
    pub fn set_envelope(&mut self, envelope: Envelope){
        self.pulse_modulator.set_envelope(envelope);
    }
    pub fn set_freq(&mut self, freq: f32){
        self.wave_gen.set_freq(freq);
    }
    pub fn set_wave_gen(&mut self, wave_gen: &'static (Fn(f32) -> f32 + Sync)){
        self.wave_gen.set_wave_gen(wave_gen);
    }
    pub fn pulse(&mut self){
        self.start();
        self.stop();
    }
    pub fn start(&mut self){
        self.pulse_modulator.start();
    }
    pub fn stop(&mut self){
        self.pulse_modulator.stop();
    }
}