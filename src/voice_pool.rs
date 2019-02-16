use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use super::voice::Voice;

pub struct VoicePool {
    workers: Vec<Worker>,
}

impl <'a> VoicePool {
    pub fn new(voices: &Vec<Voice>) -> VoicePool {
        assert!(voices.len() > 0);

        let mut workers = Vec::with_capacity(voices.len());

        for (idx, voice) in voices.iter().enumerate() {
            workers.push(Worker::new(idx, voice));
        }

        return VoicePool {
            workers,
        }
    }

    pub fn execute(&self, event: VoiceEvent, idx: usize) {
        self.workers[idx].sender.send(event).unwrap();
    }
}

enum VoiceEventType {
    ChangeFreq,
    Pulse,
    Start,
    Stop,
    SetEnvelope,
    SetWaveForm
}

struct VoiceEvent {
    id: usize,
    event: VoiceEventType,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
    sender: mpsc::Sender<VoiceEvent>,
}

impl <'a> Worker {
    pub fn new(id: usize, voice: &'a Voice) -> Worker {
        let (sender, receiver) = mpsc::channel();
        let receiver:mpsc::Receiver<VoiceEvent> = receiver;

        let thread = thread::spawn(move || {
            loop {
                let voice_event = receiver.try_recv();
                if !voice_event.is_err() {
                    // we received a message
                    match voice_event.unwrap().event {
                        VoiceEventType::ChangeFreq => {},
                        VoiceEventType::Pulse => {},
                        VoiceEventType::Start => {},
                        VoiceEventType::Stop => {},
                        VoiceEventType::SetEnvelope => {},
                        VoiceEventType::SetWaveForm => {},
                    };
                }

                println!("Worker {} got a job; executing.", id);


            }        
        });
        Worker {
            id,
            thread,
            sender
        }
    }
}