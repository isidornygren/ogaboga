use rand::{thread_rng, Rng};
use std::{
   thread,
   time::{Duration, Instant},
};
pub mod generator;

#[derive(Clone)]
pub enum SequenceStep {
   None,
   Beat,
   Freq(f32),
   Amp(f32),
   FreqAmp(f32, f32),
}

pub type Sequence = Vec<SequenceStep>;

pub struct Sequencer {
   // (Sequence, Steps per beat)
   sequences: Vec<(Sequence, u8)>,
   bpm_s:     f64,
}

impl Sequencer {
   #[inline]
   pub fn run<F>(&self, func: F)
   where
      F: Fn(usize, Option<&SequenceStep>), {
      let sleep_time = Duration::from_secs_f64(self.bpm_s);
      let mut seq_index = 0;
      let mut start_time = Instant::now();

      loop {
         for (index, (sequence, _steps_per_beat)) in self.sequences.iter().enumerate() {
            let mod_index = seq_index % sequence.len();
            func(index, sequence.get(mod_index));
         }

         seq_index += 1;
         thread::sleep(sleep_time - start_time.elapsed());
         start_time = Instant::now();
      }
   }

   #[inline]
   pub fn run_then<F, G>(&mut self, func: F, sequence_mutator: G)
   where
      F: Fn(usize, Option<&SequenceStep>),
      G: Fn(usize, &Sequence) -> Sequence, {
      let sleep_time = Duration::from_secs_f64(self.bpm_s);
      let mut seq_index = 0;
      let mut start_time = Instant::now();

      loop {
         for (index, (sequence, _steps_per_beat)) in self.sequences.iter_mut().enumerate() {
            let mod_index = seq_index % sequence.len();
            if mod_index == 0 {
               *sequence = sequence_mutator(index, sequence);
            }
            func(index, sequence.get(mod_index));
         }

         seq_index += 1;
         thread::sleep(sleep_time - start_time.elapsed());
         start_time = Instant::now();
      }
   }
}

pub struct SequencerBuilder {
   bpm:       u16,
   sequences: Vec<(Sequence, u8)>,
}

impl SequencerBuilder {
   #[must_use]
   #[inline]
   pub fn new(bpm: u16) -> Self {
      return Self {
         bpm,
         sequences: vec![],
      };
   }

   #[must_use]
   #[inline]
   pub fn add_sequence(mut self, sequence: Sequence, steps_per_beat: u8) -> Self {
      self.sequences.push((sequence, steps_per_beat));
      return self;
   }

   #[must_use]
   #[inline]
   pub fn build(self) -> Sequencer {
      return Sequencer {
         bpm_s:     (60.0 / f64::from(self.bpm)) / 2.0,
         sequences: self.sequences,
      };
   }
}
