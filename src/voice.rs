use crate::{
   pulse_modulator::PulseModulator,
   wave_generator::{WaveBox, WaveGenerator},
};

use crate::Envelope;

// #[derive(Copy)]
pub struct Voice {
   waveform: WaveBox,
   envelope: Envelope,
}

impl Voice {
   #[must_use]
   #[inline]
   pub fn new(waveform: WaveBox, envelope: Envelope) -> Self {
      return Self { waveform, envelope };
   }
}

pub struct VoiceHandler {
   wave_gen: WaveGenerator,
   pulse_modulator: PulseModulator,
   amp: f32,
}

impl VoiceHandler {
   pub fn new(voice_args: Voice, sample_rate: u32) -> Self {
      return Self {
         wave_gen: WaveGenerator::new(sample_rate, 440.0, voice_args.waveform),
         pulse_modulator: PulseModulator::new(voice_args.envelope, sample_rate as f32),
         amp: 1.0,
      };
   }

   pub fn next(&mut self) -> f32 {
      let amplitude = self.pulse_modulator.next();
      let current_wave = self.wave_gen.next();
      return (amplitude * current_wave) * self.amp;
   }

   pub fn set_envelope(&mut self, envelope: Envelope) {
      self.pulse_modulator.set_envelope(envelope);
   }

   pub fn set_freq(&mut self, freq: f32) {
      self.wave_gen.set_freq(freq);
   }

   pub fn set_amp(&mut self, amp: f32) {
      self.amp = amp.min(1.0).max(0.0);
   }

   pub fn set_waveform(&mut self, waveform: WaveBox) {
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
