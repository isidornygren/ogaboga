extern crate cpal;
extern crate rand;
extern crate noise;

mod wave_generator;
mod pulse_modulator;
mod envelope;
mod voice;
mod voice_pool;

pub mod waveforms;

pub use crate::envelope::{Envelope};
pub use crate::voice_pool::{VoicePool, VoiceEvent};
pub use crate::voice::{Voice};
