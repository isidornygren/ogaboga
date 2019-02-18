use crate::pulse_modulator::PulseModulator;
use crate::wave_generator::{WaveForm, WaveGenerator};

use crate::Envelope;

#[derive(Copy, Clone)]
pub struct Voice {
    waveform: WaveForm,
    envelope: Envelope,
}

impl Voice {
    pub fn new(waveform: WaveForm, envelope: Envelope) -> Voice {
        return Voice { waveform, envelope };
    }
}

pub struct VoiceHandler {
    wave_gen: WaveGenerator,
    pulse_modulator: PulseModulator,
}

impl VoiceHandler {
    pub fn new(voice_args: Voice, sample_rate: u32) -> VoiceHandler {
        return VoiceHandler {
            wave_gen: WaveGenerator::new(sample_rate, 440.0, voice_args.waveform),
            pulse_modulator: PulseModulator::new(voice_args.envelope, sample_rate),
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
