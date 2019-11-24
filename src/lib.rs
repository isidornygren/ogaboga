#![warn(
	clippy::all,
	clippy::restriction,
	clippy::pedantic,
	clippy::nursery,
	clippy::cargo
)]
#![allow(
	clippy::needless_return,
	clippy::missing_docs_in_private_items,
	clippy::float_arithmetic,
	clippy::clone_on_ref_ptr,
	clippy::implicit_return,
	clippy::integer_arithmetic,
	clippy::module_name_repetitions,
	clippy::else_if_without_else
)]

extern crate cpal;
extern crate rand;

mod effects;
mod envelope;
mod pulse_modulator;
pub mod sequencer;
mod voice;
mod voice_pool;
mod wave_generator;

pub mod waveforms;

pub use crate::envelope::Envelope;
pub use crate::voice::Voice;
pub use crate::voice_pool::{VoiceEvent, VoicePool, VoicePoolBuilder};
pub use crate::wave_generator::WaveForm;
