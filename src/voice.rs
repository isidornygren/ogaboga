use super::wave_generator::{WaveStruct}; 
use super::pulse_modulator::PulseModulator;
use super::envelope::Envelope;

pub struct Voice {
    waveform: WaveStruct,
    pulse_modulator: PulseModulator,
}

impl Voice {
    pub fn new(sample_rate: u32, envelope: Envelope, wave_gen: &'static (Fn(f32) -> f32 + Sync)) -> Voice {
        return Voice {
            waveform: WaveStruct::new(sample_rate, 440.0, wave_gen),
            pulse_modulator: PulseModulator::new(envelope, sample_rate),
        }
    }
    pub fn next(&mut self) -> f32 {
        let amplitude = self.pulse_modulator.next();
        let current_wave = self.waveform.next();
        return amplitude * current_wave;
    }
    pub fn set_envelope(&mut self, envelope: Envelope){
        self.pulse_modulator.set_envelope(envelope);
    }
    pub fn set_freq(&mut self, freq: f32){
        self.waveform.set_freq(freq);
    }
    pub fn set_wave_gen(&mut self, wave_gen: &'static (Fn(f32) -> f32 + Sync)){
        self.waveform.set_wave_gen(wave_gen);
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