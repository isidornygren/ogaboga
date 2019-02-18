extern crate cpal;
extern crate noise;
extern crate rand;

mod envelope;
mod pulse_modulator;
mod voice;
mod voice_pool;
mod wave_generator;

pub mod waveforms;

pub use crate::envelope::Envelope;
pub use crate::voice::Voice;
pub use crate::voice_pool::{VoiceEvent, VoicePoolBuilder};
pub use crate::wave_generator::WaveForm;
