use super::envelope::Envelope;

#[derive(Debug, Copy, Clone)]
enum Stage {
    Attack,
    Decay,
    Sustain,
    Release,
    None
}

#[derive(Debug, Copy, Clone)]
pub struct PulseModulator {
    clock: f32,
    amplitude: f32,
    envelope: Envelope,
    sample_rate: u32,
    stage: Stage,
    active: bool,
    // These are the coefficients from
    // the envelope function
    att_coef: f32,
    dec_coef: f32,
    rel_coef: f32,
}

impl PulseModulator {
    pub fn new(envelope: Envelope, sample_rate: u32) -> PulseModulator {
        let mut pulse_modulator = PulseModulator {
            clock: 0.0,
            amplitude: 0.0,
            envelope: envelope,
            sample_rate: sample_rate,
            stage: Stage::Attack,
            active: false,
            att_coef: 0.0,
            dec_coef: 0.0,
            rel_coef: 0.0,
        };
        pulse_modulator.calc_envelope_coef();
        return pulse_modulator;
    }
    pub fn set_envelope(&mut self, envelope: Envelope){
        self.envelope = envelope;
        self.calc_envelope_coef();
    }
    fn calc_envelope_coef(&mut self) {
        self.att_coef = 1.0 / (self.sample_rate as f32 * self.envelope.attack);
        self.dec_coef = 1.0 / (self.sample_rate as f32 * self.envelope.decay);
        self.rel_coef = 1.0 / (self.sample_rate as f32 * self.envelope.decay);
    }
    pub fn start(&mut self){
        self.clock = 0.0;
        self.stage = Stage::Attack;
        self.active = true;
    }
    pub fn stop(&mut self){
        self.active = false;
    }
    pub fn next(&mut self) -> f32{
        self.clock = self.clock + 1.0;
        match self.stage {
            Stage::Attack => {
                self.amplitude = self.amplitude + self.att_coef;
                if self.amplitude > 1.0 {
                    self.amplitude = 1.0;
                    self.stage = Stage::Decay;
                }
            },
            Stage::Decay => {
                self.amplitude = self.amplitude - self.dec_coef;
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
                self.amplitude = self.amplitude - self.rel_coef;
                if self.amplitude < 0.0 {
                    self.amplitude = 0.0;
                    self.stage = Stage::None;
                }
            },
            Stage::None => {}
        };
        return self.amplitude;
    }
}