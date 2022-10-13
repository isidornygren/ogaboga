use std::{thread, time::Instant};
pub mod generator;
use crate::rhythm::RhythmController;

// mod pool;
// mod sequencer_thread;

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
   sequences: Vec<Sequence>,
   rhythm_controller: Box<RhythmController>,
}

impl Sequencer {
   #[inline]
   pub fn run<F>(&mut self, func: F)
   where
      F: Fn(usize, Option<&SequenceStep>),
   {
      let mut seq_index = 0;
      let mut start_time = Instant::now();

      loop {
         for (index, sequence) in self.sequences.iter().enumerate() {
            let mod_index = seq_index % sequence.len();
            func(index, sequence.get(mod_index));
         }

         seq_index += 1;
         let sleep_time = self.rhythm_controller.step();
         thread::sleep(sleep_time - start_time.elapsed());
         start_time = Instant::now();
      }
   }

   pub fn set_sequence(&mut self, sequences: Vec<Sequence>) {
      self.sequences = sequences;
   }

   #[inline]
   pub fn run_then<F, G>(&mut self, func: F, sequence_mutator: G)
   where
      F: Fn(usize, Option<&SequenceStep>),
      G: Fn(usize, &Sequence) -> Sequence,
   {
      // let sleep_time = Duration::from_secs_f64(self.bpm_s);
      let mut seq_index = 0;
      let mut start_time = Instant::now();

      loop {
         for (index, sequence) in self.sequences.iter_mut().enumerate() {
            let mod_index = seq_index % sequence.len();
            if mod_index == 0 {
               *sequence = sequence_mutator(index, sequence);
            }
            func(index, sequence.get(mod_index));
         }

         seq_index += 1;
         let sleep_time = self.rhythm_controller.step();
         thread::sleep(sleep_time - start_time.elapsed());
         start_time = Instant::now();
      }
   }
}

pub struct SequencerBuilder {
   sequences: Vec<Sequence>,
   rhythm_controller: Box<dyn RhythmController>,
}

impl SequencerBuilder {
   #[must_use]
   #[inline]
   pub fn new(rhythm_controller: Box<dyn RhythmController>) -> Self {
      return Self {
         rhythm_controller,
         sequences: vec![],
      };
   }

   #[must_use]
   #[inline]
   pub fn add_sequence(mut self, sequence: Sequence) -> Self {
      self.sequences.push(sequence);
      return self;
   }

   #[must_use]
   #[inline]
   pub fn build(self) -> Sequencer {
      return Sequencer {
         rhythm_controller: self.rhythm_controller,
         sequences: self.sequences,
      };
   }
}
