use crate::Envelope;

#[derive(Debug, Copy, Clone)]
enum Stage {
   Attack,
   Decay,
   Sustain,
   Release,
   None,
}

#[derive(Debug, Copy, Clone)]
pub struct PulseModulator {
   clock: f32,
   amplitude: f32,
   envelope: Envelope,
   sample_rate: f32,
   stage: Stage,
   active: bool,
   // These are the coefficients from
   // the envelope function
   att_coef: f32,
   dec_coef: f32,
   rel_coef: f32,
}

impl PulseModulator {
   pub fn new(envelope: Envelope, sample_rate: f32) -> Self {
      let mut pulse_modulator = Self {
         clock: 0.0,
         amplitude: 0.0,
         envelope,
         sample_rate,
         stage: Stage::None,
         active: false,
         att_coef: 0.0,
         dec_coef: 0.0,
         rel_coef: 0.0,
      };
      pulse_modulator.calc_envelope_coef();
      return pulse_modulator;
   }

   pub fn set_envelope(&mut self, envelope: Envelope) {
      self.envelope = envelope;
      self.calc_envelope_coef();
   }

   fn calc_envelope_coef(&mut self) {
      self.att_coef = 1.0 / (self.sample_rate * self.envelope.attack);
      self.dec_coef = (1.0 - self.envelope.sustain) / (self.sample_rate * self.envelope.decay);
      self.rel_coef = self.envelope.sustain / (self.sample_rate * self.envelope.release);
   }

   pub fn start(&mut self) {
      self.clock = 0.0;
      self.stage = Stage::Attack;
      self.active = true;
   }

   pub fn stop(&mut self) {
      self.active = false;
   }

   pub fn next(&mut self) -> f32 {
      self.clock += 1.0;
      match self.stage {
         Stage::Attack => {
            self.amplitude += self.att_coef;
            if self.amplitude > 1.0 {
               self.amplitude = 1.0;
               self.stage = Stage::Decay;
            }
         }
         Stage::Decay => {
            self.amplitude -= self.dec_coef;
            if self.amplitude <= self.envelope.sustain {
               self.amplitude = self.envelope.sustain;
               self.stage = Stage::Sustain;
            }
         }
         Stage::Sustain => {
            if !self.active {
               self.stage = Stage::Release;
            }
         }
         Stage::Release => {
            self.amplitude -= self.rel_coef;
            if self.amplitude < 0.0 {
               self.amplitude = 0.0;
               self.stage = Stage::None;
            }
         }
         Stage::None => {}
      };
      return self.amplitude;
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   const ENVELOPE: Envelope = Envelope {
      attack: 1.0,
      decay: 0.2,
      sustain: 0.5,
      release: 2.0,
   };

   #[test]
   fn calculates_attack_coef() {
      let p_m = PulseModulator::new(ENVELOPE, 10.0);
      assert_eq!(p_m.att_coef, 0.1);
   }
   #[test]
   fn calculates_decay_coef() {
      let p_m = PulseModulator::new(ENVELOPE, 10.0);
      assert_eq!(p_m.dec_coef, 0.25);
   }
   #[test]
   fn calculates_release_coef() {
      let p_m = PulseModulator::new(ENVELOPE, 10.0);
      assert_eq!(p_m.rel_coef, 0.025);
   }
}
