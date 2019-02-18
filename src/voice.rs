use crate::pulse_modulator::PulseModulator;
use crate::wave_generator::{WaveForm, WaveGenerator};

use crate::Envelope;

pub struct VoiceArgs {
    wave_gen: WaveForm,
    envelope: Envelope,
}

pub struct Voice {
    wave_gen: WaveGenerator,
    pulse_modulator: PulseModulator,
}

impl Voice {
    pub fn new(sample_rate: u32, envelope: Envelope, waveform: WaveForm) -> Voice {
        return Voice {
            wave_gen: WaveGenerator::new(sample_rate, 440.0, waveform),
            pulse_modulator: PulseModulator::new(envelope, sample_rate),
        };
    }
    pub fn next(&mut self) -> f32 {
        let amplitude = self.pulse_modulator.next();
        let current_wave = self.wave_gen.next();
        return amplitude * current_wave;
    }
    pub fn set_envelope(&mut self, envelope: Envelope) {
        self.pulse_modulator.set_envelope(envelope);
    }
    pub fn set_freq(&mut self, freq: f32) {
        self.wave_gen.set_freq(freq);
    }
    pub fn set_waveform(&mut self, waveform: WaveForm) {
        self.wave_gen.set_waveform(waveform);
    }
    pub fn pulse(&mut self) {
        self.start();
        self.stop();
    }
    pub fn start(&mut self) {
        self.pulse_modulator.start();
    }
    pub fn stop(&mut self) {
        self.pulse_modulator.stop();
    }
}
