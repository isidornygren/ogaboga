use std::sync::{mpsc, mpsc::{SendError}};
use std::thread;
use super::voice::Voice;
use super::envelope::Envelope;

pub enum VoiceEvent {
    ChangeFreq(f32),
    Pulse,
    Start,
    Stop,
    SetEnvelope(Envelope),
    SetWaveForm(&'static (Fn(f32) -> f32 + Sync))
}

pub struct VoicePool {
    workers: Vec<Worker>,
}

impl VoicePool {
    pub fn new() -> VoicePool {
        return VoicePool {
            workers: vec![],
        }
    }

    pub fn add_voice(&mut self, voice: Voice) -> usize {
        self.workers.push(Worker::new(self.workers.len(), voice));
        return self.workers.len() - 1;
    }

    pub fn send(&self, event: VoiceEvent, idx: usize) -> Result<(), SendError<VoiceEvent>>{
        assert!(self.workers.len() >= idx);
        self.workers[idx].sender.send(event)
    }
}

struct Worker {
    id: usize, // id is only there so we can keep track of each worker from themselves
    thread: thread::JoinHandle<()>,
    sender: mpsc::Sender<VoiceEvent>,
}

impl Worker {
    pub fn new(id: usize, mut voice: Voice) -> Worker {
        // Build event actions
        let (sender, receiver) = mpsc::channel();
        let receiver:mpsc::Receiver<VoiceEvent> = receiver;
        // Build output variables
        let device = cpal::default_output_device().expect("Failed to get default output device");
        let format = device.default_output_format().expect("Failed to get default output format");
        let event_loop = cpal::EventLoop::new();
        let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
        event_loop.play_stream(stream_id.clone());

        let thread = thread::spawn(move || {
            event_loop.run(move |_, data| {
                let voice_event = receiver.try_recv();
                if !voice_event.is_err() {
                    // we received a message
                    match voice_event.unwrap() {
                        VoiceEvent::ChangeFreq(frequency) => {
                            voice.set_freq(frequency);
                        },
                        VoiceEvent::Pulse => {
                            voice.pulse();
                        },
                        VoiceEvent::Start => {
                            voice.start();
                        },
                        VoiceEvent::Stop => {
                            voice.stop();
                        },
                        VoiceEvent::SetEnvelope(envelope) => {
                            voice.set_envelope(envelope);
                        },
                        VoiceEvent::SetWaveForm(waveform) => {
                            voice.set_wave_gen(waveform);
                        },
                    };
                }
                // Stream data
                match data {
                    cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer) } => {
                        for sample in buffer.chunks_mut(format.channels as usize) {
                            // let value = ((next_value() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                            let value = ((voice.next() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                            for out in sample.iter_mut() {
                                *out = value;
                            }
                        }
                    },
                    cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {
                        for sample in buffer.chunks_mut(format.channels as usize) {
                            let value = (voice.next() * std::i16::MAX as f32) as i16;
                            for out in sample.iter_mut() {
                                *out = value;
                            }
                        }
                    },
                    cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
                        for sample in buffer.chunks_mut(format.channels as usize) {
                            let value = voice.next();
                            for out in sample.iter_mut() {
                                *out = value;
                            }
                        }
                    },
                    _ => (),
                }
            }); 
        });
        Worker {
            id,
            thread,
            sender
        }
    }
}