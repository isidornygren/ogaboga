use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use std::{sync::mpsc, thread};

use crate::{
    sequencer::{SequenceStep, Sequencer},
    voice::{Voice, VoiceHandler},
    wave_generator::WaveBox,
    Envelope,
};

pub struct SequencerThread {
    thread: thread::JoinHandle<()>,
    // pub sender: mpsc::Sender<SequencerEvent>,
}

impl SequencerThread {
    #[must_use]
    pub fn new(sequencer: Sequencer) -> Self {
        // Build event actions
        let (_sender, _receiver) = mpsc::channel();

        let thread = thread::spawn(move || {
            sequencer.run_then(
                |index, step| match step {
                    Some(SequenceStep::Beat) => {
                        voice_pool.send(VoiceEvent::Pulse, index).unwrap();
                    }
                    Some(SequenceStep::Freq(freq)) => {
                        voice_pool
                            .send(VoiceEvent::PulseFreq(*freq), index)
                            .unwrap();
                    }
                    _ => {}
                },
                |index, sequence| {
                    if index == base_index {
                        // only mutating the base value a little bit every time
                        return base_drum_generator.mutate(&base_drum_sequence, 0.2);
                    } else if index == hihat_index {
                        // continously mutating the value
                        return high_hat_generator.mutate(sequence, 0.5);
                    } else if index == main_index {
                        // only mutating the base value a little bit every time
                        return tune_generator.mutate(&tune_sequence, 0.2);
                    } else {
                        return sequence.clone();
                    }
                },
            )
        });
        return Self {
            thread, /*, sender*/
        };
    }
}
