use std::sync::mpsc::SendError;

use crate::voice::{
    thread::{VoiceEvent, VoiceThread},
    Voice,
};

pub struct VoicePool {
    workers: Vec<VoiceThread>,
}

impl VoicePool {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        return Self {
            workers: Vec::new(),
        };
    }

    /// Adds a voice to the voice pool and returns the index of the voice
    #[inline]
    pub fn add_voice(&mut self, voice: Voice) -> usize {
        self.workers.push(VoiceThread::new(voice));
        return self.workers.len() - 1;
    }

    #[inline]
    pub fn send(&self, event: VoiceEvent, id: usize) -> Result<(), SendError<VoiceEvent>> {
        assert!(self.workers.len() >= id);
        self.workers[id].sender.send(event)
    }
}
