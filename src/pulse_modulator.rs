use super::wave_generator::{WaveStruct, WaveGenerator};
use super::envelope::Envelope;

use rand::prelude::*;

enum Stage {
    Attack,
    Decay,
    Sustain,
    Release,
    None
}

pub struct PulseModulator<W: WaveGenerator> {
    clock: f32,
    amplitude: f32,
    target_amplitude: f32,
    envelope: Envelope,
    wave_form: WaveStruct<W>,
    stage: Stage,
}

impl <W: WaveGenerator> PulseModulator <W> {
    pub fn new(envelope: Envelope, wave_form: WaveStruct<W>) -> PulseModulator<W> {
        return PulseModulator {
            clock: 0.0,
            amplitude: 0.0,
            target_amplitude: 0.0,
            envelope: envelope,
            wave_form: wave_form,
            stage: Stage::Attack,
        }
    }
    fn start(&mut self){
        self.clock = 0.0;
        self.stage = Stage::Attack;
    }
    fn stop(&mut self){
    }
    fn pulse(&mut self){
        self.start();
        self.stop();
    }
    fn update_amplitude(&mut self) {
        let mut amplitude = self.target_amplitude;
        match self.stage {
            Stage::Attack => {
                amplitude = amplitude + self.envelope.att_coef;
                if amplitude > 1.0 {
                    amplitude = 1.0;
                    self.stage = Stage::Decay;
                }
            },
            Stage::Decay => {
                amplitude = amplitude - self.envelope.dec_coef;
                if amplitude < self.envelope.sustain {
                    amplitude = self.envelope.sustain;
                    self.stage = Stage::Sustain;
                }
            },
            Stage::Sustain => {
                self.stage = Stage::Release;
            },
            Stage::Release => {
                amplitude = amplitude - self.envelope.rel_coef;
                if amplitude < 0.0 {
                    amplitude = 0.0;
                    self.stage = Stage::None;
                }
            },
            Stage::None => {}
        };
        self.target_amplitude = amplitude;
        self.amplitude = amplitude;
        // self.amplitude = self.amplitude + (self.target_amplitude - self.amplitude).min(0.000001);
    }
    pub fn next(&mut self) -> f32 {
        self.clock = self.clock + 1.0;
        if self.clock > 20000.0 {
            self.pulse();
            self.wave_form.change_freq(80.0 + rand::thread_rng().gen::<f32>() * 440.0);
        }
        self.update_amplitude();
        return self.wave_form.next() * self.target_amplitude;
    }
}