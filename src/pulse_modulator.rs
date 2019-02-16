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

pub struct PulseModulator {
    clock: f32,
    amplitude: f32,
    envelope: Envelope,
    wave_form: WaveStruct,
    stage: Stage,
    active: bool,
}

impl PulseModulator {
    pub fn new(envelope: Envelope, wave_form: WaveStruct) -> PulseModulator {
        return PulseModulator {
            clock: 0.0,
            amplitude: 0.0,
            envelope: envelope,
            wave_form: wave_form,
            stage: Stage::Attack,
            active: false
        }
    }
    fn start(&mut self){
        self.clock = 0.0;
        self.stage = Stage::Attack;
        self.active = true;
    }
    fn stop(&mut self){
        self.active = false;
    }
    fn pulse(&mut self){
        self.start();
        self.stop();
    }
    fn update_amplitude(&mut self) {
        match self.stage {
            Stage::Attack => {
                self.amplitude = self.amplitude + self.envelope.att_coef;
                if self.amplitude > 1.0 {
                    self.amplitude = 1.0;
                    self.stage = Stage::Decay;
                }
            },
            Stage::Decay => {
                self.amplitude = self.amplitude - self.envelope.dec_coef;
                if self.amplitude < self.envelope.sustain {
                    self.amplitude = self.envelope.sustain;
                    self.stage = Stage::Sustain;
                }
            },
            Stage::Sustain => {
                if !self.active {
                    self.stage = Stage::Release;
                }
            },
            Stage::Release => {
                self.amplitude = self.amplitude - self.envelope.rel_coef;
                if self.amplitude < 0.0 {
                    self.amplitude = 0.0;
                    self.stage = Stage::None;
                }
            },
            Stage::None => {}
        };
    }
    pub fn next(&mut self) -> f32 {
        self.clock = self.clock + 1.0;
        if self.clock > 80000.0 {
            self.wave_form.change_freq(80.0 + rand::thread_rng().gen::<f32>() * 440.0);
            self.pulse();
        }
        self.update_amplitude();
        return self.wave_form.next() * self.amplitude;
    }
}