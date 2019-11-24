use rand::{thread_rng, Rng};
use std::{
	thread,
	time::{Duration, Instant},
};

#[derive(Clone)]
pub enum SequenceStep {
	None,
	Beat,
	Freq(f32),
	Amp(f32),
	FreqAmp(f32, f32),
}

pub struct Sequence {
	pub steps: Vec<SequenceStep>,
}

impl Sequence {
	#[inline]
	pub fn new(sequence_size: usize) -> Self {
		return Self {
			steps: vec![SequenceStep::None; sequence_size],
		};
	}
}

pub struct SequenceBuilder {
	pub sequence: Sequence,
	pub half_step_chance: Option<f64>,
}

impl SequenceBuilder {
	#[inline]
	pub fn new(sequence_size: usize) -> Self {
		return Self {
			sequence: Sequence::new(sequence_size * 2),
			half_step_chance: None,
		};
	}

	#[inline]
	pub const fn allow_half_steps(mut self, chance: f64) -> Self {
		self.half_step_chance = Some(chance);
		return self;
	}

	#[inline]
	pub fn beat_sin(mut self, fraq: f32, min: f32, max: f32, period_offset: f32) -> Self {
		let period = fraq * self.sequence.steps.len() as f32;
		let mut rng = thread_rng();

		for (index, step) in self.sequence.steps.iter_mut().enumerate() {
			let is_half_step = ((index + 1) % 2) == 0;
			if is_half_step {
				if let Some(half_step_chance) = self.half_step_chance {
					let current = index as f32 % period;
					let current_chance =
						((((current * (std::f32::consts::PI * 2.0) / period + period_offset).sin() + 1.0)
							/ 2.0) * (max - min)
							+ min) * half_step_chance as f32;
					*step = if rng.gen_bool(f64::from(current_chance)) {
						SequenceStep::Beat
					} else {
						SequenceStep::None
					}
				} else {
					*step = SequenceStep::None;
				}
			} else {
				let current = index as f32 % period;
				let current_chance = (((current * (std::f32::consts::PI * 2.0 + period_offset) / period)
					.sin() + 1.0)
					/ 2.0) * (max - min)
					+ min;
				*step = if rng.gen_bool(f64::from(current_chance)) {
					SequenceStep::Beat
				} else {
					SequenceStep::None
				}
			}
		}

		return self;
	}

	#[inline]
	pub fn build(self) -> Sequence {
		return self.sequence;
	}
}

pub struct Sequencer {
	sequences: Vec<Sequence>,
	bpm_s: f64,
}

impl Sequencer {
	#[inline]
	pub fn run<F>(&self, func: F)
	where
		F: Fn(usize, Option<&SequenceStep>),
	{
		let sleep_time = Duration::from_secs_f64(self.bpm_s);
		let mut seq_index = 0;
		let mut start_time = Instant::now();

		loop {
			for (index, sequence) in self.sequences.iter().enumerate() {
				let mod_index = seq_index % sequence.steps.len();
				func(index, sequence.steps.get(mod_index));
			}

			seq_index += 1;
			thread::sleep(sleep_time - start_time.elapsed());
			start_time = Instant::now();
		}
	}
}

pub struct SequencerBuilder {
	bpm: u16,
	sequences: Vec<Sequence>,
}

impl SequencerBuilder {
	#[inline]
	pub fn new(bpm: u16) -> Self {
		return Self {
			bpm,
			sequences: vec![],
		};
	}

	#[inline]
	pub fn add_sequence(mut self, sequence: Sequence) -> Self {
		self.sequences.push(sequence);
		return self;
	}

	#[inline]
	pub fn build(self) -> Sequencer {
		return Sequencer {
			bpm_s: (60.0 / self.bpm as f64) / 2.0,
			sequences: self.sequences,
		};
	}
}
