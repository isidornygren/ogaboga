extern crate ogaboga;
extern crate rand;

use ogaboga::{
   scale::C_MAJOR,
   sequencer::{
      generator::{BeatGenerator, SequenceGenerator, TuneGenerator},
      SequenceStep, SequencerBuilder,
   },
   waveforms::{freq_noise, one_bit_noise, sawtooth_wave, square_wave, triangle_wave, white_noise},
   Envelope, Voice, VoiceEvent, VoicePoolBuilder,
};
use rand::Rng;
use std::{thread, time};

const BPM: u16 = 120;

fn main() {
   // Initiate the voice pool that we will initiate voices in
   let mut voice_pool_builder = VoicePoolBuilder::new();
   voice_pool_builder = voice_pool_builder.with_voice(Voice::new(
      Box::new(triangle_wave),
      Envelope::new(0.001, 0.1, 0.1, 0.1),
   ));
   voice_pool_builder = voice_pool_builder.with_voice(Voice::new(
      freq_noise(0.25),
      Envelope::new(0.001, 0.1, 0.1, 0.1),
   ));
   voice_pool_builder = voice_pool_builder.with_voice(Voice::new(
      Box::new(sawtooth_wave),
      Envelope::new(0.1, 0.6, 0.6, 0.6),
   ));
   let voice_pool = voice_pool_builder.build();
   voice_pool.send(VoiceEvent::ChangeFreq(50.0), 0).unwrap();
   voice_pool.send(VoiceEvent::ChangeFreq(500.0), 1).unwrap();
   // voice_pool.send(VoiceEvent::ChangeAmp(1.0), 1).unwrap();
   voice_pool.send(VoiceEvent::ChangeAmp(0.2), 2).unwrap();

   let sequencer_builder = SequencerBuilder::new(BPM);

   let base_drum_generator = BeatGenerator::new()
      .period_fraction(1.0 / 2.0)
      .chance_range(0.0, 0.5)
      .half_step_chance(Some(0.5));

   let high_hat_generator = BeatGenerator::new()
      .period_fraction(1.0 / 2.0)
      .chance_range(0.0, 0.5)
      .period_offset(std::f32::consts::PI)
      .half_step_chance(Some(0.5));

   let tune_generator = TuneGenerator::new(C_MAJOR.to_vec());

   let base_drum_sequence = base_drum_generator.generate(8);
   let high_hat_sequence = high_hat_generator.generate(8);
   let tune_sequence = tune_generator.generate(4);

   let mut sequencer = sequencer_builder
      .add_sequence(base_drum_sequence.clone(), 2)
      .add_sequence(high_hat_sequence.clone(), 2)
      .add_sequence(tune_sequence.clone(), 1)
      .build();

   sequencer.run_then(
      |index, step| match step {
         Some(SequenceStep::Beat) => {
            voice_pool.send(VoiceEvent::Pulse, index).unwrap();
         },
         Some(SequenceStep::Freq(freq)) => {
            voice_pool
               .send(VoiceEvent::PulseFreq(*freq), index)
               .unwrap();
         },
         _ => {},
      },
      |index, sequence| {
         if index == 0 {
            // only mutating the base value a little bit every time
            return base_drum_generator.mutate(&base_drum_sequence, 0.2);
         } else if index == 1 {
            // continously mutating the value
            return high_hat_generator.mutate(sequence, 0.5);
         } else if index == 2 {
            // only mutating the base value a little bit every time
            return tune_generator.mutate(&tune_sequence, 0.2);
         } else {
            return sequence.clone();
         }
      },
   );
}
