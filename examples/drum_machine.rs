extern crate ogaboga;
extern crate rand;

use ogaboga::{
	sequencer::{SequenceBuilder, SequenceStep, SequencerBuilder},
	waveforms::{sawtooth_wave, square_wave, triangle_wave, white_noise},
	Envelope, Voice, VoiceEvent, VoicePoolBuilder,
};
use rand::Rng;
use std::{thread, time};

const BPM: u16 = 320;
const SEQUENCE_SIZE: usize = 16;

fn main() {
	// Initiate the voice pool that we will initiate voices in
	let mut voice_pool_builder = VoicePoolBuilder::new();
	voice_pool_builder =
		voice_pool_builder.with_voice(Voice::new(&f32::sin, Envelope::new(0.005, 0.1, 0.25, 0.25)));
	voice_pool_builder =
		voice_pool_builder.with_voice(Voice::new(&white_noise, Envelope::new(0.01, 0.1, 0.1, 0.1)));
	voice_pool_builder =
		voice_pool_builder.with_voice(Voice::new(&white_noise, Envelope::new(0.01, 0.1, 0.5, 1.0)));
	let voice_pool = voice_pool_builder.build();
	voice_pool
		.send(VoiceEvent::ChangeFreq(87.30706), 0)
		.unwrap();
	voice_pool.send(VoiceEvent::ChangeFreq(75.0), 1).unwrap();
	voice_pool.send(VoiceEvent::ChangeAmp(0.1), 1).unwrap();
	voice_pool.send(VoiceEvent::ChangeAmp(0.1), 2).unwrap();

	let sequencer_builder = SequencerBuilder::new(BPM);

	let sequencer = sequencer_builder
		.add_sequence(
			SequenceBuilder::new(SEQUENCE_SIZE / 2)
				// .allow_half_steps(0.25)
				.beat_sin(1.0 / 4.0, 0.1, 0.75, 0.0)
				.build(),
		)
		.add_sequence(
			SequenceBuilder::new(SEQUENCE_SIZE)
				.allow_half_steps(0.25)
				.beat_sin(1.0 / 4.0, 0.1, 0.75, std::f32::consts::PI)
				.build(),
		)
		/* .add_sequence(
			SequenceBuilder::new(SEQUENCE_SIZE)
				.beat_sin(1.0 / 2.0, 0.0, 0.5)
				.build(),
		) */
		.build();

	sequencer.run(|index, step| match step {
		Some(SequenceStep::Beat) => {
			voice_pool.send(VoiceEvent::Pulse, index).unwrap();
		}
		_ => {}
	});
}
