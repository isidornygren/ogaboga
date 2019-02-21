use std::sync::{mpsc, mpsc::SendError};
use std::thread;

use crate::voice::{Voice, VoiceHandler};
use crate::Envelope;
use crate::WaveForm;

pub enum VoiceEvent {
    ChangeFreq(f32),
    ChangeAmp(f32),
    Pulse,
    Start,
    Stop,
    SetEnvelope(Envelope),
    SetWaveForm(WaveForm),
}

pub struct VoicePoolBuilder {
    voices: Vec<Voice>,
}

impl VoicePoolBuilder {
    pub fn new() -> VoicePoolBuilder {
        return VoicePoolBuilder { voices: vec![] };
    }
    pub fn with_voice(mut self, voice: Voice) -> VoicePoolBuilder {
        self.voices.push(voice);
        return self;
    }
    pub fn build(self) -> VoicePool {
        return VoicePool::new(self.voices.into_iter().map(|v| Worker::new(v)).collect());
    }
}

pub struct VoicePool {
    workers: Vec<Worker>,
}

impl VoicePool {
    fn new(workers: Vec<Worker>) -> VoicePool {
        return VoicePool { workers };
    }

    pub fn send(&self, event: VoiceEvent, id: usize) -> Result<(), SendError<VoiceEvent>> {
        assert!(self.workers.len() >= id);
        self.workers[id].sender.send(event)
    }
}

#[allow(dead_code)]
struct Worker {
    thread: thread::JoinHandle<()>,
    sender: mpsc::Sender<VoiceEvent>,
}

impl Worker {
    pub fn new(voice: Voice) -> Worker {
        // Build event actions
        let (sender, receiver) = mpsc::channel();
        let receiver: mpsc::Receiver<VoiceEvent> = receiver;
        // Build output variables
        let device = cpal::default_output_device()
            .expect("Failed to get default output device building a voice pool.");
        let format = device
            .default_output_format()
            .expect("Failed to get default output format building a voice pool.");
        let event_loop = cpal::EventLoop::new();
        let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
        event_loop.play_stream(stream_id.clone());
        // build the voice
        let mut voice_handler = VoiceHandler::new(voice, format.sample_rate.0);

        let thread = thread::spawn(move || {
            event_loop.run(move |_, data| {
                let voice_event = receiver.try_recv();
                if !voice_event.is_err() {
                    // we received a message
                    match voice_event.unwrap() {
                        VoiceEvent::ChangeFreq(frequency) => {
                            voice_handler.set_freq(frequency);
                        }
                        VoiceEvent::ChangeAmp(amplitude) => {
                            voice_handler.set_amp(amplitude);
                        }
                        VoiceEvent::Pulse => {
                            voice_handler.pulse();
                        }
                        VoiceEvent::Start => {
                            voice_handler.start();
                        }
                        VoiceEvent::Stop => {
                            voice_handler.stop();
                        }
                        VoiceEvent::SetEnvelope(envelope) => {
                            voice_handler.set_envelope(envelope);
                        }
                        VoiceEvent::SetWaveForm(waveform) => {
                            voice_handler.set_waveform(waveform);
                        }
                    };
                }
                // Stream data
                match data {
                    cpal::StreamData::Output {
                        buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer),
                    } => {
                        for sample in buffer.chunks_mut(format.channels as usize) {
                            // let value = ((next_value() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                            let value =
                                ((voice_handler.next() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                            for out in sample.iter_mut() {
                                *out = value;
                            }
                        }
                    }
                    cpal::StreamData::Output {
                        buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer),
                    } => {
                        for sample in buffer.chunks_mut(format.channels as usize) {
                            let value = (voice_handler.next() * std::i16::MAX as f32) as i16;
                            for out in sample.iter_mut() {
                                *out = value;
                            }
                        }
                    }
                    cpal::StreamData::Output {
                        buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
                    } => {
                        for sample in buffer.chunks_mut(format.channels as usize) {
                            let value = voice_handler.next();
                            for out in sample.iter_mut() {
                                *out = value;
                            }
                        }
                    }
                    _ => (),
                }
            });
        });
        return Worker { thread, sender };
    }
}
