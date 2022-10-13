use crate::{
   scale::notes::Note,
   sequencer::{Sequence, SequenceStep},
};
use rand::{thread_rng, Rng};

pub trait SequenceGenerator {
   fn generate_step(&self, index: usize, len: usize) -> SequenceStep;

   #[inline]
   fn generate(&self, sequence_length: usize) -> Sequence {
      return vec![SequenceStep::None; sequence_length * 2]
         .iter()
         .enumerate()
         .map(|(index, _)| self.generate_step(index, sequence_length))
         .collect();
   }

   /// Mutates a sequence with the same rules as the sequence generator.
   /// Iterates through the sequence and generates a random number for every
   /// Sequence node and will generate a new node if the generated number is
   /// below chance.
   #[inline]
   fn mutate(&self, sequence: &Sequence, chance: f64) -> Sequence {
      let mut rng = thread_rng();
      return sequence
         .iter()
         .enumerate()
         .map(|(index, step)| {
            if rng.gen_bool(chance) {
               return self.generate_step(index, sequence.len());
            } else {
               return step.clone();
            }
         })
         .collect();
   }
}

pub struct BeatGenerator {
   fraction: f32,
   min: f32,
   max: f32,
   period_offset: f32,
}

impl BeatGenerator {
   #[must_use]
   #[inline]
   pub const fn new() -> Self {
      Self {
         fraction: 0.25,
         min: 0.0,
         max: 1.0,
         period_offset: 0.0,
      }
   }

   #[must_use]
   #[inline]
   pub const fn period_fraction(mut self, fraction: f32) -> Self {
      self.fraction = fraction;
      self
   }

   #[must_use]
   #[inline]
   pub const fn chance_range(mut self, chance_min: f32, chance_max: f32) -> Self {
      self.min = chance_min;
      self.max = chance_max;
      self
   }

   #[must_use]
   #[inline]
   pub const fn period_offset(mut self, period_offset: f32) -> Self {
      self.period_offset = period_offset;
      self
   }

   #[must_use]
   #[inline]
   pub fn get_current_chance(&self, index: usize, len: usize) -> f32 {
      let period = self.fraction * len as f32;
      let current = index as f32 % period;
      return (((current * (std::f32::consts::PI.mul_add(2.0, self.period_offset)) / period)
         .sin()
         + 1.0)
         / 2.0)
         .mul_add(self.max - self.min, self.min);
   }
}

impl SequenceGenerator for BeatGenerator {
   #[inline]
   fn generate_step(&self, index: usize, len: usize) -> SequenceStep {
      let mut rng = thread_rng();
      let current_chance = self.get_current_chance(index, len);

      return if rng.gen_bool(f64::from(current_chance)) {
         SequenceStep::Beat
      } else {
         SequenceStep::None
      };
   }
}

pub struct TuneGenerator {
   scale: Vec<Note>,
   octave: u16,
}

impl TuneGenerator {
   #[must_use]
   #[inline]
   pub const fn new(scale: Vec<Note>, octave: u16) -> Self {
      return Self { scale, octave };
   }
}

impl SequenceGenerator for TuneGenerator {
   #[inline]
   fn generate_step(&self, _index: usize, _len: usize) -> SequenceStep {
      let mut rng = thread_rng();
      return if rng.gen_bool(0.9) {
         SequenceStep::Freq(
            self.scale[rng.gen_range(0, self.scale.len())].get_freq(self.octave) as f32,
         )
      } else {
         SequenceStep::None
      };
   }
}
