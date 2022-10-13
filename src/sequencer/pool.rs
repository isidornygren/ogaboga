use std::sync::mpsc::SendError;

use crate::sequencer::{sequencer_thread::SequencerThread, Sequencer};

pub struct SequencerPool {
    workers: Vec<SequencerThread>,
}

impl SequencerPool {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        return Self {
            workers: Vec::new(),
        };
    }

    #[inline]
    pub fn add_sequencer(&mut self, sequencer: Sequencer) -> usize {
        self.workers.push(SequencerThread::new(sequencer));
        return self.workers.len() - 1;
    }
}
