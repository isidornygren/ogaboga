extern crate noise;
extern crate ogaboga;
extern crate rand;

use ogaboga::waveforms::{sawtooth_wave, square_wave, triangle_wave, white_noise};
use ogaboga::{Envelope, Voice, VoiceEvent, VoicePoolBuilder};
use rand::Rng;
use std::{thread, time};

const VOICES: usize = 3;
const BPM: u16 = 360;
const SEQUENCER_SIZE: usize = 4;
const BPM_MS: u64 = ((60.0 / BPM as f32) * 1000.0) as u64;
const NODE_PERCENTAGE: f32 = 0.5;

const RANDOMIZE_AFTER: u8 = 2;

fn main() {
	// Initiate the voice pool that we will initiate voices in
	let mut voice_pool_builder = VoicePoolBuilder::new();
	let sequence = vec![None; SEQUENCER_SIZE];
	let mut sequencer = vec![sequence.clone(), sequence.clone(), sequence.clone()];

	for voice_index in 0..VOICES {
		for sequence_index in 0..sequence.len() {
			if rand::thread_rng().gen::<f32>() > NODE_PERCENTAGE {
				sequencer[voice_index][sequence_index] =
					Some(rand::thread_rng().gen::<f32>() * 500.0 + 200.0);
			}
		}
	}

	voice_pool_builder =
		voice_pool_builder.with_voice(Voice::new(&f32::sin, Envelope::new(0.005, 0.1, 0.25, 0.25)));
	voice_pool_builder =
		voice_pool_builder.with_voice(Voice::new(&white_noise, Envelope::new(0.1, 0.1, 0.5, 1.0)));
	voice_pool_builder =
		voice_pool_builder.with_voice(Voice::new(&white_noise, Envelope::new(0.01, 0.1, 0.1, 0.5)));
	let voice_pool = voice_pool_builder.build();
	voice_pool
		.send(VoiceEvent::ChangeFreq(87.30706), 0)
		.unwrap();
	voice_pool.send(VoiceEvent::ChangeFreq(75.0), 1).unwrap();
	voice_pool.send(VoiceEvent::ChangeAmp(0.0), 1).unwrap();
	voice_pool.send(VoiceEvent::ChangeAmp(0.1), 2).unwrap();
	// voice_pool.send(VoiceEvent::ChangeFreq(500.0), 2).unwrap();

	let mut seq_index = 0;

	let mut randomize_new = 0;

	loop {
		let sleep_time = time::Duration::from_millis(BPM_MS);
		thread::sleep(sleep_time);

		for voice_index in 0..VOICES {
			if let Some(_) = sequencer[voice_index][seq_index] {
				voice_pool.send(VoiceEvent::Pulse, voice_index).unwrap();
			}
		}

		seq_index += 1;
		if seq_index == sequence.len() {
			seq_index = 0;
			randomize_new += 1;

			if randomize_new == RANDOMIZE_AFTER {
				randomize_new = 0;
				sequencer = vec![sequence.clone(), sequence.clone(), sequence.clone()];

				for voice_index in 0..VOICES {
					for sequence_index in 0..sequence.len() {
						if rand::thread_rng().gen::<f32>() > NODE_PERCENTAGE {
							sequencer[voice_index][sequence_index] =
								Some(rand::thread_rng().gen::<f32>() * 500.0 + 200.0);
						}
					}
				}
			}
		}
	}
}
