extern crate ogaboga;
extern crate rand;

use ogaboga::{
	sequencer::{SequenceBuilder, SequenceStep, SequencerBuilder},
	waveforms::{freq_noise, one_bit_noise, sawtooth_wave, square_wave, triangle_wave, white_noise},
	Envelope, Voice, VoiceEvent, VoicePoolBuilder,
};
use rand::Rng;
use std::{thread, time};

const BPM: u16 = 320;

fn main() {
	// Initiate the voice pool that we will initiate voices in
	let mut voice_pool_builder = VoicePoolBuilder::new();
	voice_pool_builder = voice_pool_builder.with_voice(Voice::new(
		Box::new(triangle_wave),
		Envelope::new(0.001, 0.1, 0.1, 0.1),
	));
	voice_pool_builder = voice_pool_builder.with_voice(Voice::new(
		freq_noise(0.1),
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

	let sequencer = sequencer_builder
		.add_sequence(
			SequenceBuilder::new(7)
				// .allow_half_steps(0.25)
				.beat_sin(1.0 / 4.0, 0.1, 0.75, 0.0)
				.build(),
		)
		.add_sequence(
			SequenceBuilder::new(13)
				.beat_sin(1.0 / 8.0, 0.25, 0.5, std::f32::consts::PI)
				.build(),
		)
		.add_sequence(
			SequenceBuilder::new(17)
				.allow_half_steps(0.25)
				.beat_sin(1.0 / 4.0, 0.1, 0.75, 0.0)
				.build(),
		)
		.build();

	sequencer.run(|index, step| match step {
		Some(SequenceStep::Beat) => {
			voice_pool.send(VoiceEvent::Pulse, index).unwrap();
		}
		_ => {}
	});
}
