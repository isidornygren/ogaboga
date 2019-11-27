extern crate ogaboga;
extern crate rand;

use ogaboga::{
	sequencer::{
		generator::{BeatGenerator, SequenceGenerator},
		SequenceStep, SequencerBuilder,
	},
	waveforms::{freq_noise, one_bit_noise, sawtooth_wave, square_wave, triangle_wave, white_noise},
	Envelope, Voice, VoiceEvent, VoicePoolBuilder,
};
use rand::Rng;
use std::{thread, time};

const BPM: u16 = 240;

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
		freq_noise(1.75),
		Envelope::new(0.001, 0.1, 0.1, 0.1),
	));
	let voice_pool = voice_pool_builder.build();
	voice_pool.send(VoiceEvent::ChangeFreq(50.0), 0).unwrap();
	voice_pool.send(VoiceEvent::ChangeFreq(1000.0), 1).unwrap();
	// voice_pool.send(VoiceEvent::ChangeAmp(1.0), 1).unwrap();
	voice_pool.send(VoiceEvent::ChangeAmp(0.2), 2).unwrap();

	let sequencer_builder = SequencerBuilder::new(BPM);

	let base_drum_generator = BeatGenerator::new()
		.period_fraction(1.0 / 4.0)
		.chance_range(0.1, 0.75);

	let high_hat_generator = BeatGenerator::new()
		.period_fraction(1.0 / 4.0)
		.chance_range(0.25, 0.5)
		.period_offset(std::f32::consts::PI)
		.half_step_chance(Some(0.5));

	let base_drum_sequence = base_drum_generator.generate(8);
	let high_hat_sequence = high_hat_generator.generate(8);

	let mut sequencer = sequencer_builder
		.add_sequence(base_drum_sequence.clone())
		.add_sequence(high_hat_sequence.clone())
		.build();

	sequencer.run_then(
		|index, step| match step {
			Some(SequenceStep::Beat) => {
				voice_pool.send(VoiceEvent::Pulse, index).unwrap();
			}
			_ => {}
		},
		|index, sequence| {
			if index == 0 {
				return base_drum_generator.mutate(&base_drum_sequence, 0.2);
			} else if index == 1 {
				return high_hat_generator.mutate(&high_hat_sequence, 0.2);
			} else {
				return sequence.clone();
			}
		},
	);
}
