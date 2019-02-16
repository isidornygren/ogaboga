extern crate cpal;
extern crate rand;
extern crate noise;

mod wave_generator;
mod pulse_modulator;
mod envelope;
mod voice;
mod voice_pool;

use self::wave_generator::{WaveStruct, square_wave, sawtooth_wave, triangle_wave};
use self::envelope::Envelope;
use self::voice::Voice;

use std::thread;

fn main() {
    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device.default_output_format().expect("Failed to get default output format");
    let event_loop = cpal::EventLoop::new();

    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone());

    let sample_rate = format.sample_rate.0;
    let mut voice = Voice::new(sample_rate, Envelope::new(
            1.0,
            0.5,
            0.5,
            0.5
            ), &sawtooth_wave);
    voice.start();

    event_loop.run(move |_, data| {
        // wave_struct.change_freq(440.0 + wave_struct.current_clock);
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
}
