use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use std::{sync::mpsc, thread};

use crate::{
    voice::{Voice, VoiceHandler},
    wave_generator::WaveBox,
    Envelope,
};

pub enum VoiceEvent {
    ChangeFreq(f32),
    ChangeAmp(f32),
    PulseFreq(f32),
    Pulse,
    Start,
    Stop,
    SetEnvelope(Envelope),
    SetWaveForm(WaveBox),
}

pub struct VoiceThread {
    thread: thread::JoinHandle<()>,
    pub sender: mpsc::Sender<VoiceEvent>,
}

impl VoiceThread {
    #[must_use]
    pub fn new(voice: Voice) -> Self {
        // Build event actions
        let (sender, receiver) = mpsc::channel();
        let receiver: mpsc::Receiver<VoiceEvent> = receiver;
        // Build output variables
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("Failed to get default output device building a voice pool.");
        let format = device
            .default_output_format()
            .expect("Failed to get default output format building a voice pool.");
        let event_loop = host.event_loop();
        let stream_id = event_loop
            .build_output_stream(&device, &format)
            .expect("Could not build output stream");
        event_loop.play_stream(stream_id);

        // build the voice
        let mut voice_handler = VoiceHandler::new(voice, format.sample_rate.0);

        let thread = thread::spawn(move || {
            event_loop.run(move |stream_id, stream_result| {
                let stream_data = match stream_result {
                    Ok(data) => data,
                    Err(err) => {
                        eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                        return;
                    }
                    _ => return,
                };
                let voice_event = receiver.try_recv();
                if voice_event.is_ok() {
                    match voice_event.expect("Could not unwrap voice event") {
                        VoiceEvent::ChangeFreq(freq) => {
                            voice_handler.set_freq(freq);
                        }
                        VoiceEvent::ChangeAmp(amp) => {
                            voice_handler.set_amp(amp);
                        }
                        VoiceEvent::PulseFreq(freq) => {
                            voice_handler.set_freq(freq);
                            voice_handler.pulse();
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
                match stream_data {
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
        return Self { thread, sender };
    }
}
